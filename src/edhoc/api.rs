//! Structs used in the API.

use alloc::vec::Vec;
use core::result::Result;
use x25519_dalek_ng::{PublicKey, SharedSecret, StaticSecret};
use super::{
    cose,
    error::{EarlyError, Error, OwnError, OwnOrPeerError},
    util::{self, Message1, Message2, Message3,Message4},
};


// Party U constructs ---------------------------------------------------------

/// The structure providing all operations for Party I.
pub struct PartyI<S: PartyIState>(pub S);

// Necessary stuff for session types
pub trait PartyIState {}
impl PartyIState for Msg1Sender {}
impl PartyIState for Msg2Receiver {}
impl PartyIState for Msg2Verifier {}
impl PartyIState for Msg3Sender {}
impl PartyIState for Msg4ReceiveVerify {}



pub struct Msg1Sender {
    ead_1: Option<Vec<u8>>,
    c_i : Vec<u8>,
    secret: StaticSecret,
    x_i: PublicKey,
    static_secret: StaticSecret,
    static_public: PublicKey,
    kid: Vec<u8>,
}

impl PartyI<Msg1Sender> {
    /// Creates a new `PartyI` ready to build the first message.
    ///
    /// # Arguments
    /// * `c_i` - The chosen connection identifier.
    /// * ead - external auth data
    /// * `ecdh_secret` - The ECDH secret to use for this protocol run. Ephemeral
    /// * `stat_priv` - The private ed25519derivePRKauthentication key.
    /// * `stat_public`, which is called 'id_cred_x in edho 14 .
    /// * `kid` - The key ID by which the other party is able to retrieve

    pub fn new(
        c_i: Vec<u8>,
        ead_1: Option<Vec<u8>>,
        ecdh_secret: [u8; 32],
        stat_priv: StaticSecret,
        stat_pub: PublicKey,
        kid: Vec<u8>,
    ) -> PartyI<Msg1Sender> {

        let secret = StaticSecret::from(ecdh_secret);
        // and from that build the corresponding public key
        let x_i = PublicKey::from(&secret);



        // Combine the authentication key pair for convenience
         PartyI(Msg1Sender {
            ead_1,
            c_i,
            secret,
            x_i,
            static_secret:stat_priv,
            static_public:stat_pub,
            kid,
        })
    }

    /// Returns the bytes of the first message.
    ///

    pub fn generate_message_1(
        self,
        method: u8,
        suites: u8,
    ) -> Result<(Vec<u8>, PartyI<Msg2Receiver>), EarlyError> {
        // Encode the necessary information into the first message
        let msg_1 = Message1 {
            method,
            suite: suites,
            x_i: self.0.x_i.as_bytes().to_vec(), // sending PK as vector
            c_i : self.0.c_i,
            ead_1 : self.0.ead_1,
        };
        // Get CBOR sequence for message
        let msg_1_seq = util::serialize_message_1(&msg_1)?;
        // Copy for returning
        let msg_1_bytes = msg_1_seq.clone();
        Ok((
            msg_1_bytes,
            PartyI(Msg2Receiver {
                i_ecdh_ephemeralsecret: self.0.secret,
                stat_priv: self.0.static_secret,
                stat_pub: self.0.static_public,
                kid: self.0.kid,
                msg_1_seq,
            }),
        ))
    }
}
/// Contains the state to receive the second message.
pub struct Msg2Receiver {
    i_ecdh_ephemeralsecret: StaticSecret,
    stat_priv : StaticSecret,
    stat_pub : PublicKey,
    kid: Vec<u8>,
    msg_1_seq: Vec<u8>,
}



impl PartyI<Msg2Receiver> {
    /// Returns the key ID of the other party's public authentication key, and the state for verification 
    pub fn unpack_message_2_return_kid_ead(
        self,
        msg_2: Vec<u8>,
    ) -> Result<(Vec<u8>, Vec<u8>, Option<Vec<u8>>,PartyI<Msg2Verifier>), OwnOrPeerError> {

        util::fail_on_error_message(&msg_2)?;


        let msg_2 = util::deserialize_message_2(&msg_2)?;

        // cosntructing ephemeral keypair

        // Constructing shared secret for initiator 
        let mut x_r_bytes = [0; 32];
        x_r_bytes.copy_from_slice(&msg_2.ephemeral_key_r[..32]);
        let r_ephemeral_pk = x25519_dalek_ng::PublicKey::from(x_r_bytes);


       let shared_secret_0 = self.0.i_ecdh_ephemeralsecret.diffie_hellman(&r_ephemeral_pk);
        

        // reconstructing keystream2
        let c_r_cpy = msg_2.c_r.clone();
        let th_2 = util::compute_th_2(self.0.msg_1_seq, &msg_2.c_r, r_ephemeral_pk)?;
        let (prk_2e,prk_2e_hkdf) = util::extract_prk(None, shared_secret_0.as_bytes())?;


        let keystream2 = util::edhoc_kdf(
                                            &prk_2e_hkdf, 
                                            &th_2, 
                                            "KEYSTREAM_2",
                                            &[],
                                            msg_2.ciphertext_2.len(), 
                                            )?;


        let decryptedlaintext = util::xor(&keystream2, &msg_2.ciphertext_2)?;

        let (r_kid,mac_2,ead_2 ) = util::extract_plaintext(decryptedlaintext)?;


        Ok((
            r_kid.clone(),
            c_r_cpy,
            ead_2.clone(),
            PartyI(Msg2Verifier {
                i_ecdh_ephemeralsecret : self.0.i_ecdh_ephemeralsecret,
                stat_priv: self.0.stat_priv,
                stat_pub : self.0.stat_pub,
                kid: self.0.kid,
                msg_2,
                mac_2,
                ead_2,
                prk_2e,
                th_2,
                r_kid,
                r_ephemeral_pk,


            }),
        ))

    }

    pub fn unpack_message_2_return_kid(
        self,
        msg_2: Vec<u8>,
    ) -> Result<(Vec<u8>, Vec<u8>,PartyI<Msg2Verifier>), OwnOrPeerError> {
        let (kid, c_r , _ead, msg2_receiver) = self.unpack_message_2_return_kid_ead(msg_2)?;

        Ok((kid,c_r, msg2_receiver))
    }


}



/// Contains the state to verify the second message.
pub struct Msg2Verifier {
    i_ecdh_ephemeralsecret : StaticSecret,
    stat_priv : StaticSecret,
    stat_pub : PublicKey,
    kid: Vec<u8>,
    msg_2: Message2,
    mac_2: Vec<u8>,
    ead_2 : Option<Vec<u8>>,
    prk_2e : Vec<u8>,
    th_2: Vec<u8>,
    r_kid: Vec<u8>,
    r_ephemeral_pk : PublicKey,
}


impl PartyI<Msg2Verifier> {
    /// Checks the authenticity of the second message with the other party's
    /// public authentication key.
    pub fn verify_message_2(
        self,
        r_public_static_bytes: &[u8],
    ) -> Result<PartyI<Msg3Sender>, OwnError> {

        // build cred_x and id_cred_x (for responder party)
        let id_cred_r = cose::build_id_cred_x(&self.0.r_kid)?;

        let cred_r = cose::serialize_cred_x(r_public_static_bytes,&self.0.r_kid )?; 
        // Generating static public key of initiator
        let mut statkey_r_bytes = [0; 32];
        statkey_r_bytes.copy_from_slice(&r_public_static_bytes[..32]);
        let r_public_static = x25519_dalek_ng::PublicKey::from(statkey_r_bytes);

        // Generating shared secret 1 for initiator

        let shared_secret_1 = self.0.i_ecdh_ephemeralsecret.diffie_hellman(&r_public_static);

        // generating prk_3

        let (prk_3e2m,prk_3e2m_hkdf) = util::extract_prk(Some(&self.0.prk_2e)
            ,shared_secret_1.as_bytes())?;

        
        let mac_2 = util::create_mac_with_kdf(&prk_3e2m_hkdf, 
            util::EDHOC_MAC /8, 
            &self.0.th_2, 
            "MAC_2", 
            id_cred_r, 
            cred_r,
            &self.0.ead_2)?;
      
        if self.0.mac_2 != mac_2{
            return Err(Error::BadMac.into())
        }
        

        Ok(PartyI(Msg3Sender{
            i_stat_priv : self.0.stat_priv,
            i_stat_pub : self.0.stat_pub,
            r_ephemeral_pk: self.0.r_ephemeral_pk,
            i_kid : self.0.kid,
            msg_2 : self.0.msg_2,
            th_2 : self.0.th_2,
            prk_3e2m_hkdf,
            prk_3e2m
        }))
    }
}

/// Contains the state to build the third message.
pub struct Msg3Sender {
    i_stat_priv : StaticSecret,
    i_stat_pub : PublicKey,

    r_ephemeral_pk : PublicKey, 
    i_kid: Vec<u8>,
    msg_2: Message2,
    th_2: Vec<u8>,
    prk_3e2m_hkdf :  hkdf::Hkdf<sha2::Sha256>,
    prk_3e2m : Vec<u8>,

}

impl PartyI<Msg3Sender> {
    /// Returns the bytes of the third message, as well as the OSCORE master
    /// secret and the OSCORE master salt.
    pub fn generate_message_3(
        self,
        ead_3: Option<Vec<u8>>,
    ) -> Result<(PartyI<Msg4ReceiveVerify>,Vec<u8>), OwnError> {

        //first making necessary copies:

        // Build the COSE header map identifying the public authentication key
        let id_cred_i = cose::build_id_cred_x(&self.0.i_kid)?;
        // Build the COSE_Key containing our public authentication key
        let cred_i = cose::serialize_cred_x(&self.0.i_stat_pub.to_bytes(), &self.0.i_kid)?;

        let shared_secret_2 = self.0.i_stat_priv.diffie_hellman(&self.0.r_ephemeral_pk);
        
        
        // transcript hash 3

        let th_3 = util::compute_th_3(
            &self.0.th_2, 
            &self.0.msg_2.ciphertext_2)?;

            
        let (_,prk_4x3m_hkdf) = util::extract_prk(
            Some(&self.0.prk_3e2m),
             shared_secret_2.as_bytes())?;

        let mac_3 = util::create_mac_with_kdf(
            &self.0.prk_3e2m_hkdf, 
            util::EDHOC_MAC/8, 
            &th_3,  
            "MAC_3",
             id_cred_i, 
             cred_i,
            &ead_3)?;

        
        let k_3 = util::edhoc_kdf(
            &self.0.prk_3e2m_hkdf, 
            &th_3, 
            "K_3",
            b"",
            util::CCM_KEY_LEN / 8)?;
            
        let iv_3 = util::edhoc_kdf(
            &self.0.prk_3e2m_hkdf, 
            &th_3, 
            "IV_3",
            b"",
            util::CCM_NONCE_LEN / 8)?;
        let p = util::build_plaintext(&self.0.i_kid, &mac_3,ead_3)?;

        let ad = cose::build_ad(&th_3)?;

        // Constructing ciphertext:
        let ciphertext_3 = util::aead_seal(&k_3, &iv_3, &p, &ad)?;
        let ciphertext_3_cpy = ciphertext_3.clone();
        let msg_3 = Message3 {ciphertext: ciphertext_3};
        let msg_3_seq = util::serialize_message_3(&msg_3)?;



        // now computing the values needed for sck and rck
        let th_4 = util::compute_th_4(&th_3, &ciphertext_3_cpy)?;


        let master_secret = util::edhoc_exporter(
            &prk_4x3m_hkdf,
            &th_4,
            "OSCORE_Master_Secret",
            b"",
            util::CCM_KEY_LEN/8, //going from bits to bytes

        )?;

        let master_salt = util::edhoc_exporter(
            &prk_4x3m_hkdf,
            &th_4,
            "OSCORE_Master_Salt",
            b"",
            util::SALT_LENGTH/8,//going from bits to bytes
        )?;

        Ok((PartyI(Msg4ReceiveVerify {
            prk_4x3m_hkdf,
            th_4,
            master_salt,
            master_secret
        }),msg_3_seq))
    }
}


pub struct Msg4ReceiveVerify {
    prk_4x3m_hkdf : hkdf::Hkdf<sha2::Sha256>,
    th_4 : Vec<u8>,
    master_secret : Vec<u8>,
    master_salt : Vec<u8>,
}

    /// Handle message four, and return output keying material and ead, if wanted
    ///
    /// # Arguments
    /// * `msg4_seq` msg 4 as bytes
    /// Outputs (sck,rck,rk,ead)


impl PartyI<Msg4ReceiveVerify> {
    pub fn handle_message_4_ead(
        self,
        msg4_seq : Vec<u8>,
    ) -> Result<(Vec<u8>, Vec<u8>,Vec<u8>,Vec<u8>), OwnOrPeerError> {


        util::fail_on_error_message(&msg4_seq)?;
        let msg4 = util::deserialize_message_4(&msg4_seq)?;


        let k_4 = util::edhoc_exporter(
            &self.0.prk_4x3m_hkdf,
            &self.0.th_4,
            "EDHOC_K_4",
            b"",
            util::CCM_KEY_LEN /8, //going from bits to bytes
        )?;

        let iv_4 = util::edhoc_exporter(
            &self.0.prk_4x3m_hkdf,
            &self.0.th_4,
            "EDHOC_IV_4",
            b"",
            util::CCM_NONCE_LEN/8 , //going from bits to bytes
        )?;
        let ad = cose::build_ad(&self.0.th_4)?;
        let plaintext = util::aead_open(&k_4, &iv_4, &msg4.ciphertext, &ad)?;

        let ead : Vec<u8> = Vec::new();
        if !plaintext.is_empty(){
            let ead = util::deserialize_ead(&plaintext)?;
        }


        let sck = util::extract_expand(
            &self.0.master_secret,
            &self.0.master_salt, 
            "DOWNLINK", 
            32, 
            )?;

        let rck = util::extract_expand(
            &self.0.master_secret,
            &self.0.master_salt,
            "UPLINK", 
            32,  
            )?;

        let rk = util::extract_expand(
            &self.0.master_secret,
            &self.0.master_salt,
            "RK0", 
            32,  
            )?;

    

        Ok((sck,rck,rk,ead))
    }

    pub fn handle_message_4(
        self,
        msg4_seq : Vec<u8>,
    ) -> Result<(Vec<u8>, Vec<u8>,Vec<u8>), OwnOrPeerError> {
        let (sck,rck,rk,_) = self.handle_message_4_ead(msg4_seq)?;
        Ok((sck,rck,rk))
    }


}
// Party V constructs ---------------------------------------------------------

/// The structure providing all operations for Party V.
pub struct PartyR<S: PartyRState>(pub S);
// Necessary stuff for session types
pub trait PartyRState {}
impl PartyRState for Msg1Receiver {}
impl PartyRState for Msg2Sender {}
impl PartyRState for Msg3Receiver {}
impl PartyRState for Msg3verifier {}
impl PartyRState for Msg4Sender {}

/// Contains the state to receive the first message.
/// 
pub struct Msg1Receiver {
    secret: StaticSecret,
    x_r: PublicKey,
    stat_priv: StaticSecret,
    stat_pub: PublicKey,
    kid: Vec<u8>,
}

impl PartyR<Msg1Receiver> {
    /// Creates a new `PartyR` ready to receive the first message.
    ///
    /// # Arguments
    /// * `c_v` - The chosen connection identifier.
    /// * `ecdh_secret` - The ECDH secret to use for this protocol run.
    /// * `auth_private` - The private ed25519 authentication key.
    /// * `auth_public` - The public ed25519 authentication key.
    /// * `kid` - The key ID by which the other party is able to retrieve
    ///   `auth_public`.
    pub fn new(
        ecdh_secret: [u8; 32],
        stat_priv: StaticSecret,
        stat_pub: PublicKey,
        kid: Vec<u8>,
    ) -> PartyR<Msg1Receiver> {
        // From the secret bytes, create the DH secret
        let secret = StaticSecret::from(ecdh_secret);
        // and from that build the corresponding public key
        let x_r = PublicKey::from(&secret);
        // Combine the authentication key pair for convenience

        PartyR(Msg1Receiver {
            secret,
            x_r,
            stat_priv,
            stat_pub,
            kid,
        })
    }

    /// Processes the first message, return ead
    pub fn handle_message_1_ead(
        self,
        msg_1: Vec<u8>,
    ) -> Result<(PartyR<Msg2Sender>,Vec<u8>,Option<Vec<u8>>), OwnError> {
        // Alias this
        let msg_1_seq = msg_1;
        // Decode the first message

        let msg_1 = util::deserialize_message_1(&msg_1_seq)?;

        // Verify that the selected suite is supported
        
        if msg_1.suite != 3 {
            return Err(Error::UnsupportedSuite.into())
        }

        // Use U's public key to generate the ephemeral shared secret
        let mut ed_key_bytes = [0; 32];
        ed_key_bytes.copy_from_slice(&msg_1.x_i[..32]);
        let i_public = x25519_dalek_ng::PublicKey::from(ed_key_bytes);

        // generating shared secret at responder
        let shared_secret_0 = self.0.secret.diffie_hellman(&i_public.clone());
        
        let shared_secret_1 = self.0.stat_priv.diffie_hellman(&i_public);



        Ok((PartyR(Msg2Sender {
                ecdh_r_secret : self.0.secret,
            shared_secret_0,
            shared_secret_1,
            x_r: self.0.x_r,
            stat_pub: self.0.stat_pub,
            r_kid: self.0.kid,
            msg_1_seq,
        }),
        msg_1.c_i,
        msg_1.ead_1))
    }
    /// Processes the first message.
    pub fn handle_message_1(
        self,
        msg_1: Vec<u8>,
    ) -> Result<(PartyR<Msg2Sender>,Vec<u8>), OwnError> {
        // simply wrapping the handling of message 1, but not returning ead, allowing R to discard ead
        let (msg2_sender, c_i, _ead) = self.handle_message_1_ead(msg_1)?;

        Ok((msg2_sender, c_i))

    }
}

/// Contains the state to build the second message.
///  shared_secret_0 : the first shared secret created from ephemeral keys only
/// shared_secret_1 : The second shared secret, created only from I's  ephemeral key,R and static key
/// shared_secret_2 : the third shared secret, created only from I's  static key, and R's ephemeral key
/// (this is from the side of I)
pub struct Msg2Sender {
    ecdh_r_secret: StaticSecret,
    shared_secret_0: SharedSecret,
    shared_secret_1: SharedSecret,
    x_r: PublicKey,
    stat_pub : PublicKey,
    r_kid: Vec<u8>,
    msg_1_seq: Vec<u8>,
}

impl PartyR<Msg2Sender> {
    /// Returns the bytes of the second message.
    pub fn generate_message_2(
        self,
        c_r : Vec<u8>,
        ead_2 : Option<Vec<u8>>,
    ) -> Result<(Vec<u8>, PartyR<Msg3Receiver>),OwnOrPeerError> {
            // first we need to build the id_cred_r from the kid
            let id_cred_r = cose::build_id_cred_x(&self.0.r_kid)?;

            // We now build the cred_x using the public key, and kid value
            let cred_r = cose::serialize_cred_x(&self.0.stat_pub.to_bytes(),&self.0.r_kid )?; 

            let th_2 = util::compute_th_2(self.0.msg_1_seq, &c_r, self.0.x_r)?;

            let (prk_2e,prk_2e_hkdf) = util::extract_prk(None, self.0.shared_secret_0.as_bytes())?;

            let (prk_3e2m,prk_3e2m_hkdf) = util::extract_prk(Some(&prk_2e),self.0.shared_secret_1.as_bytes())?;

            let mac_2 = util::create_mac_with_kdf(
                &prk_3e2m_hkdf, 
                util::EDHOC_MAC/8, 
                &th_2, 
                "MAC_2", 
                id_cred_r, 
                cred_r,
                &ead_2)?;


            
            let plaintext_encoded = util::build_plaintext(&self.0.r_kid, &mac_2,ead_2)?;

            let keystream2 = util::edhoc_kdf(
                &prk_2e_hkdf, 
                &th_2, 
                "KEYSTREAM_2",
                b"",
                plaintext_encoded.len(), 
                )?;
            let ciphertext_2 = util::xor(&keystream2, &plaintext_encoded)?;


            let msg_2 = Message2 {
                ephemeral_key_r : self.0.x_r.as_bytes().to_vec(),
                c_r,
                ciphertext_2,
            };




            let msg2_seq = util::serialize_message_2(&msg_2)?;

            Ok((msg2_seq, 
                PartyR(Msg3Receiver {
                    r_ecdh_secret: self.0.ecdh_r_secret,
                    prk_3e2m_hkdf,
                    prk_3e2m,
                    msg_2,
                    th_2,
                }),
            ))

        
    }
}

/// Contains the state to receive the third message.
pub struct Msg3Receiver {
    r_ecdh_secret : StaticSecret,
    prk_3e2m_hkdf  : hkdf::Hkdf<sha2::Sha256>,
    prk_3e2m : Vec<u8>,
    msg_2: Message2,
    th_2: Vec<u8>,
}

impl PartyR<Msg3Receiver> {
    /// Returns the kid of the other party, and the state to verify
    pub fn unpack_message_3_return_kid_ead(
        self,
        msg_3_seq: Vec<u8>,
    ) -> Result<(PartyR<Msg3verifier>, Vec<u8>,Option<Vec<u8>>), OwnOrPeerError> {
        util::fail_on_error_message(&msg_3_seq)?;
        // first, relevant copies:

        let msg_3 = util::deserialize_message_3(&msg_3_seq)?;


        let th_3 = util::compute_th_3(
            &self.0.th_2, 
            &self.0.msg_2.ciphertext_2)?;


        let k_3 = util::edhoc_kdf(
            &self.0.prk_3e2m_hkdf, 
            &th_3, 
            "K_3",
            b"",
            util::CCM_KEY_LEN / 8)?;

        let iv_3 = util::edhoc_kdf(
            &self.0.prk_3e2m_hkdf, 
            &th_3, 
            "IV_3",
            b"",
            util::CCM_NONCE_LEN / 8)?;
        
        let ad = cose::build_ad(&th_3)?;



        let p = util::aead_open(
            &k_3, 
            &iv_3, 
            &msg_3.ciphertext, 
            &ad)?;
        
        let (r_kid, mac3,ead_3) = util::extract_plaintext(p)?;

        Ok((PartyR(Msg3verifier{
            r_ecdh_secret : self.0.r_ecdh_secret,
            prk_3e2m_hkdf : self.0.prk_3e2m_hkdf,
            prk_3e2m : self.0.prk_3e2m,
            msg_3,
            kid : r_kid.clone(),
            mac3,
            ead_3 : ead_3.clone(),
            th_3,
        }),
        r_kid,
        ead_3))
    }

    pub fn unpack_message_3_return_kid(
        self,
        msg_3_seq: Vec<u8>,
    ) -> Result<(PartyR<Msg3verifier>, Vec<u8>), OwnOrPeerError> {

        let (msg_3_verifier, kid, _ead_3) = self.unpack_message_3_return_kid_ead(msg_3_seq)?;
        Ok((
        msg_3_verifier,
        kid))
    }
}


pub struct Msg3verifier {
    r_ecdh_secret : StaticSecret,
    prk_3e2m_hkdf : hkdf::Hkdf<sha2::Sha256>,
    prk_3e2m : Vec<u8>,
    msg_3 : Message3,
    kid : Vec<u8>,
    mac3 : Vec<u8>,
    ead_3 : Option<Vec<u8>>,
    th_3: Vec<u8>,
}
impl PartyR<Msg3verifier> {
    /// Returns the key ID of the other party's public authentication key.
    pub fn verify_message_3(
        self,
        i_public_static_bytes: &[u8],
    ) -> Result<(PartyR<Msg4Sender>, Vec<u8>, Vec<u8>,Vec<u8>), OwnOrPeerError> {
        let mut statkey_i_bytes = [0; 32];
        statkey_i_bytes.copy_from_slice(&i_public_static_bytes[..32]);
        let i_public_static = x25519_dalek_ng::PublicKey::from(statkey_i_bytes);
            
        let shared_secret_2 = self.0.r_ecdh_secret.diffie_hellman(&i_public_static);
    

        let id_cred_i = cose::build_id_cred_x(&self.0.kid)?;

        let cred_i = cose::serialize_cred_x(&i_public_static.to_bytes(), &self.0.kid)?;
        let mac_3_initiator = util::create_mac_with_kdf(
            &self.0.prk_3e2m_hkdf, 
            util::EDHOC_MAC/8, 
            &self.0.th_3,  
            "MAC_3",
             id_cred_i, 
             cred_i,
            &self.0.ead_3)?;

        if mac_3_initiator != self.0.mac3{
           return  Err(Error::BadMac.into())
            }

        let th_4 = util::compute_th_4(&self.0.th_3, &self.0.msg_3.ciphertext)?;

        let (_,prk_4x3m_hkdf) = util::extract_prk(
                Some(&self.0.prk_3e2m),
                 shared_secret_2.as_bytes())?;
    

        let master_secret = util::edhoc_exporter(
                     &prk_4x3m_hkdf,
                     &th_4,
                    "OSCORE_Master_Secret",
                    b"",
                    util::CCM_KEY_LEN / 8, //going from bits to bytes
                )?;
       let master_salt = util::edhoc_exporter(
                    &prk_4x3m_hkdf,
                    &th_4,
                    "OSCORE_Master_Salt",
                    b"",
                    util::SALT_LENGTH / 8,//going from bits to bytes
                    
                )?;
        
        let sck = util::extract_expand(
                    &master_secret,
                    &master_salt, 
                    "DOWNLINK", 
                    32, 
                    )?;
        
       let rck = util::extract_expand(
                    &master_secret,
                    &master_salt, 
                    "UPLINK", 
                    32,  
                    )?;
        
        let rk = util::extract_expand(
                    &master_secret,
                    &master_salt, 
                    "RK0", 
                    32,  
                    )?;
        


        Ok((PartyR(Msg4Sender{
            prk_4x3m_hkdf,
            th_4,
            }),
        sck,
        rck,
        rk))

    }

}
/// Contains the state to verify the third message.
pub struct Msg4Sender {
    prk_4x3m_hkdf :hkdf::Hkdf<sha2::Sha256>,
    th_4 : Vec<u8>,
}


impl PartyR<Msg4Sender> {
    pub fn generate_message_4(
        self,
        ead_4 :Option<Vec<u8>>,
    ) -> Result< Vec<u8>, OwnOrPeerError> {


        let k_4 = util::edhoc_exporter(
            &self.0.prk_4x3m_hkdf,
            &self.0.th_4,
            "EDHOC_K_4",
            b"",
            util::CCM_KEY_LEN/8 , //going from bits to bytes
        )?;

        let iv_4 = util::edhoc_exporter(
            &self.0.prk_4x3m_hkdf,
            &self.0.th_4,
            "EDHOC_IV_4",
            b"",
            util::CCM_NONCE_LEN /8, //going from bits to bytes
        )?;

        let ad = cose::build_ad(&self.0.th_4)?;


        let tmp : Vec<u8>;
        let p = match ead_4 {
            Some(x) => { 
                tmp = util::serialize_ead(&x)?;
                &tmp
            },
            None => "".as_bytes(),
        };




        let ciphertext_4 = util::aead_seal(&k_4, &iv_4, p, &ad)?;

        let msg4 = Message4 {
            ciphertext : ciphertext_4,
        };
        let msg4_seq = util::serialize_message_4(&msg4)?;

        Ok(msg4_seq)
    }
}
