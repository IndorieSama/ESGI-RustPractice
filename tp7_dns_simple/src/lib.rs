use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Write};

/// Structure représentant l'en-tête DNS selon RFC 1035
#[derive(Debug, Clone)]
pub struct DnsHeader {
    pub id: u16,          // Identifiant de la requête
    pub flags: u16,       // Drapeaux (QR, Opcode, AA, TC, RD, RA, Z, RCODE)
    pub qdcount: u16,     // Nombre de questions
    pub ancount: u16,     // Nombre de réponses
    pub nscount: u16,     // Nombre d'enregistrements d'autorité
    pub arcount: u16,     // Nombre d'enregistrements additionnels
}

/// Structure représentant une question DNS
#[derive(Debug, Clone)]
pub struct DnsQuestion {
    pub qname: String,    // Nom de domaine demandé
    pub qtype: u16,       // Type de requête (A=1, AAAA=28, etc.)
    pub qclass: u16,      // Classe (IN=1 pour Internet)
}

/// Structure représentant une réponse DNS
#[derive(Debug, Clone)]
pub struct DnsAnswer {
    pub name: String,     // Nom de domaine
    pub rtype: u16,       // Type d'enregistrement
    pub rclass: u16,      // Classe
    pub ttl: u32,         // Durée de vie en secondes
    pub rdlength: u16,    // Longueur des données
    pub rdata: Vec<u8>,   // Données de la réponse
}

/// Structure principale du message DNS
#[derive(Debug, Clone)]
pub struct DnsMessage {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsAnswer>,
}

impl DnsHeader {
    /// Crée un nouvel en-tête DNS
    pub fn new(id: u16, is_response: bool) -> Self {
        let mut flags = 0u16;
        if is_response {
            flags |= 0x8000; // Bit QR à 1 pour une réponse
        }
        flags |= 0x0100; // Bit RD (Recursion Desired) à 1
        
        DnsHeader {
            id,
            flags,
            qdcount: 0,
            ancount: 0,
            nscount: 0,
            arcount: 0,
        }
    }

    /// Sérialise l'en-tête DNS en bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.write_u16::<BigEndian>(self.id).unwrap();
        bytes.write_u16::<BigEndian>(self.flags).unwrap();
        bytes.write_u16::<BigEndian>(self.qdcount).unwrap();
        bytes.write_u16::<BigEndian>(self.ancount).unwrap();
        bytes.write_u16::<BigEndian>(self.nscount).unwrap();
        bytes.write_u16::<BigEndian>(self.arcount).unwrap();
        bytes
    }

    /// Désérialise un en-tête DNS depuis des bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor = Cursor::new(data);
        Ok(DnsHeader {
            id: cursor.read_u16::<BigEndian>()?,
            flags: cursor.read_u16::<BigEndian>()?,
            qdcount: cursor.read_u16::<BigEndian>()?,
            ancount: cursor.read_u16::<BigEndian>()?,
            nscount: cursor.read_u16::<BigEndian>()?,
            arcount: cursor.read_u16::<BigEndian>()?,
        })
    }
}

impl DnsQuestion {
    /// Crée une nouvelle question DNS
    pub fn new(domain: String, qtype: u16) -> Self {
        DnsQuestion {
            qname: domain,
            qtype,
            qclass: 1, // IN (Internet)
        }
    }

    /// Encode un nom de domaine au format DNS (avec longueurs)
    pub fn encode_domain_name(domain: &str) -> Vec<u8> {
        let mut encoded = Vec::new();
        
        for label in domain.split('.') {
            if !label.is_empty() {
                encoded.push(label.len() as u8);
                encoded.extend_from_slice(label.as_bytes());
            }
        }
        encoded.push(0); // Terminateur null
        encoded
    }

    /// Décode un nom de domaine depuis le format DNS
    pub fn decode_domain_name(data: &[u8], offset: usize) -> Result<(String, usize), std::io::Error> {
        let mut domain = String::new();
        let mut pos = offset;
        
        loop {
            if pos >= data.len() {
                return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Données insuffisantes"));
            }
            
            let length = data[pos] as usize;
            pos += 1;
            
            if length == 0 {
                break;
            }
            
            if pos + length > data.len() {
                return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Données insuffisantes"));
            }
            
            if !domain.is_empty() {
                domain.push('.');
            }
            
            domain.push_str(&String::from_utf8_lossy(&data[pos..pos + length]));
            pos += length;
        }
        
        Ok((domain, pos))
    }

    /// Sérialise la question en bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        // Encode le nom de domaine
        let encoded_name = Self::encode_domain_name(&self.qname);
        bytes.extend_from_slice(&encoded_name);
        
        // Ajoute le type et la classe
        bytes.write_u16::<BigEndian>(self.qtype).unwrap();
        bytes.write_u16::<BigEndian>(self.qclass).unwrap();
        
        bytes
    }
}

impl DnsAnswer {
    /// Crée une nouvelle réponse DNS pour une adresse IPv4
    pub fn new_a_record(domain: String, ip: [u8; 4], ttl: u32) -> Self {
        DnsAnswer {
            name: domain,
            rtype: 1, // Type A
            rclass: 1, // Classe IN
            ttl,
            rdlength: 4,
            rdata: ip.to_vec(),
        }
    }

    /// Sérialise la réponse en bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        // Encode le nom de domaine
        let encoded_name = DnsQuestion::encode_domain_name(&self.name);
        bytes.extend_from_slice(&encoded_name);
        
        // Ajoute type, classe, TTL et longueur des données
        bytes.write_u16::<BigEndian>(self.rtype).unwrap();
        bytes.write_u16::<BigEndian>(self.rclass).unwrap();
        bytes.write_u32::<BigEndian>(self.ttl).unwrap();
        bytes.write_u16::<BigEndian>(self.rdlength).unwrap();
        
        // Ajoute les données de la réponse
        bytes.extend_from_slice(&self.rdata);
        
        bytes
    }
}

impl DnsMessage {
    /// Crée une nouvelle requête DNS
    pub fn new_query(id: u16, domain: String, qtype: u16) -> Self {
        let mut header = DnsHeader::new(id, false);
        header.qdcount = 1;
        
        let question = DnsQuestion::new(domain, qtype);
        
        DnsMessage {
            header,
            questions: vec![question],
            answers: vec![],
        }
    }

    /// Crée une nouvelle réponse DNS
    pub fn new_response(query: &DnsMessage) -> Self {
        let mut header = DnsHeader::new(query.header.id, true);
        header.qdcount = query.header.qdcount;
        
        DnsMessage {
            header,
            questions: query.questions.clone(),
            answers: vec![],
        }
    }

    /// Sérialise le message DNS complet en bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        // En-tête
        bytes.extend_from_slice(&self.header.to_bytes());
        
        // Questions
        for question in &self.questions {
            bytes.extend_from_slice(&question.to_bytes());
        }
        
        // Réponses
        for answer in &self.answers {
            bytes.extend_from_slice(&answer.to_bytes());
        }
        
        bytes
    }

    /// Désérialise un message DNS depuis des bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
        if data.len() < 12 {
            return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Données insuffisantes pour l'en-tête"));
        }

        let header = DnsHeader::from_bytes(&data[0..12])?;
        let mut pos = 12;
        let mut questions = Vec::new();

        // Parse les questions
        for _ in 0..header.qdcount {
            if pos >= data.len() {
                return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Données insuffisantes pour les questions"));
            }

            let (domain, new_pos) = DnsQuestion::decode_domain_name(data, pos)?;
            pos = new_pos;

            if pos + 4 > data.len() {
                return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Données insuffisantes pour le type/classe"));
            }

            let mut cursor = Cursor::new(&data[pos..pos + 4]);
            let qtype = cursor.read_u16::<BigEndian>()?;
            let qclass = cursor.read_u16::<BigEndian>()?;
            pos += 4;

            questions.push(DnsQuestion {
                qname: domain,
                qtype,
                qclass,
            });
        }

        // Parse les réponses
        let mut answers = Vec::new();
        for _ in 0..header.ancount {
            if pos >= data.len() {
                break; // Pas assez de données pour les réponses
            }

            // Décode le nom de domaine de la réponse
            let (domain, new_pos) = DnsQuestion::decode_domain_name(data, pos)?;
            pos = new_pos;

            if pos + 10 > data.len() {
                break; // Pas assez de données pour type, classe, TTL, rdlength
            }

            let mut cursor = Cursor::new(&data[pos..pos + 10]);
            let rtype = cursor.read_u16::<BigEndian>()?;
            let rclass = cursor.read_u16::<BigEndian>()?;
            let ttl = cursor.read_u32::<BigEndian>()?;
            let rdlength = cursor.read_u16::<BigEndian>()?;
            pos += 10;

            if pos + rdlength as usize > data.len() {
                break; // Pas assez de données pour rdata
            }

            let rdata = data[pos..pos + rdlength as usize].to_vec();
            pos += rdlength as usize;

            answers.push(DnsAnswer {
                name: domain,
                rtype,
                rclass,
                ttl,
                rdlength,
                rdata,
            });
        }

        Ok(DnsMessage {
            header,
            questions,
            answers,
        })
    }
}

/// Types de requêtes DNS constants
pub const DNS_TYPE_A: u16 = 1;     // Adresse IPv4
pub const DNS_TYPE_AAAA: u16 = 28; // Adresse IPv6
pub const DNS_TYPE_CNAME: u16 = 5; // Nom canonique