use revm::primitives::Bytes;
use tiny_keccak::{Hasher, Keccak};

pub enum AbiType {
    Uint,
    Int,
    Address,
}

impl AbiType {
    fn len(&self) -> usize {
        match self {
            AbiType::Uint | AbiType::Int => 32,
            AbiType::Address => 20,
        }
    }
}

pub enum AbiValueType {
    Uint([u8; 32]),
    Int([u8; 32]),
    Address([u8; 20]),
}

impl AbiValueType {
    fn from(value: &str, data: &[u8]) -> AbiValueType {
        if value.ends_with(']') {
            todo!("no array for now");
        }
        if value.ends_with(')') {
            todo!("no tuple for now");
        }

        // TODO should fill 0's at the end to avoid out-of-bounds access
        if value.starts_with("uint") {
            AbiValueType::Uint(Self::sanitize(data, 32).try_into().unwrap())
        } else if value.starts_with("int") {
            AbiValueType::Int(Self::sanitize(data, 32).try_into().unwrap())
        } else if value == "address" {
            AbiValueType::Address(Self::sanitize(data, 20).try_into().unwrap())
        } else {
            unreachable!()
        }
    }

    fn sanitize(data: &[u8], max: usize) -> Vec<u8> {
        match data.len() {
            len if len > max => panic!("shoot!"),
            len if len < max => {
                let mut zeros: Vec<_> = (len..max).map(|_| 0).collect();
                zeros.extend_from_slice(data);
                zeros
            }
            _ => data.to_vec(),
        }
    }

    fn inner(&self) -> &[u8] {
        match self {
            AbiValueType::Uint(inner) => inner,
            AbiValueType::Int(inner) => inner,
            AbiValueType::Address(inner) => inner,
        }
    }
}

pub fn abi_decode(data: &Bytes, types: Vec<AbiType>) -> Vec<u8> {
    let data_vec = data.to_vec();
    let mut data_slice = data_vec.as_slice();
    let tot_len: usize = types.iter().map(|ty| ty.len()).sum();
    assert_eq!(data_slice.len(), tot_len);
    types
        .into_iter()
        .flat_map(|ty| data_slice.take(..(ty.len())).unwrap().to_vec())
        .collect()
}

pub fn abi_encode(types: Vec<AbiValueType>) -> Vec<u8> {
    types
        .into_iter()
        .flat_map(|abi_ty| match abi_ty {
            AbiValueType::Uint(data) | AbiValueType::Int(data) => data.to_vec(),
            AbiValueType::Address(data) => data.to_vec(),
        })
        .collect()
}

// doesn't support tuples **yet**
pub fn abi_encode_signature(signature: &str, values: Vec<Vec<u8>>) -> Vec<u8> {
    assert!(signature.ends_with(')'));
    let par_pos = signature
        .chars()
        .enumerate()
        .find_map(|(i, c)| if c == '(' { Some(i) } else { None })
        .expect("should contain `(`");
    let sig_inner = &signature[(par_pos + 1)..(signature.len() - 1)];
    let sig = get_sig(signature);

    let types = if !sig_inner.is_empty() {
        sig_inner
            .split(',')
            .enumerate()
            .map(|(i, ty)| AbiValueType::from(ty, &values[i]))
            .collect()
    } else {
        Default::default()
    };

    [sig.as_ref(), &abi_encode(types)].concat()
}

pub fn get_sig(signature: &str) -> [u8; 4] {
    let mut keccak = Keccak::v256();
    keccak.update(signature.as_bytes());
    let mut sig_raw = [0u8; 32];
    keccak.finalize(&mut sig_raw);
    sig_raw[0..4].try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::{abi_encode_signature, get_sig};

    #[test]
    fn good_sig() {
        assert_eq!(
            get_sig("transfer(uint256,address)"),
            [0xb7, 0x76, 0x0c, 0x8f]
        );
    }

    #[test]
    fn encode_with_sig() {
        assert_eq!(abi_encode_signature("pwn()", vec![]), get_sig("pwn()"));
        assert_eq!(
            abi_encode_signature("transfer(uint256,address)", vec![vec![0u8], vec![0]]),
            [
                [0xb7, 0x76, 0x0c, 0x8f].to_vec(),
                [0; 32].to_vec(),
                [0; 20].to_vec()
            ]
            .concat()
        );
    }
}
