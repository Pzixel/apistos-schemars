use crate::gen::SchemaGenerator;
use crate::schema::*;
use crate::JsonSchema;
use std::borrow::Cow;
use alloy_primitives::aliases::*;
use alloy_primitives::Address;

macro_rules! schema_uint {
    ($t:ty) => {
        impl JsonSchema for $t {
            no_ref_schema!();
        
            fn schema_name() -> String {
                stringify!($t).to_string()
            }
        
            fn schema_id() -> Cow<'static, str> {
                Cow::Borrowed(std::any::type_name::<$t>())
            }
        
            fn json_schema(_: &mut SchemaGenerator) -> Schema {
                SchemaObject {
                    instance_type: Some(InstanceType::String.into()),
                    ..Default::default()
                }
                .into()
            }
        }
    };
}

macro_rules! schema_fixed_hash {
    ($t:ty,$len_in_chars:literal) => {
        impl JsonSchema for $t {
            no_ref_schema!();

            fn schema_name() -> String {
                stringify!($t).to_string()
            }

            fn schema_id() -> Cow<'static, str> {
                Cow::Borrowed(std::any::type_name::<$t>())
            }

            fn json_schema(_: &mut SchemaGenerator) -> Schema {
                SchemaObject {
                    instance_type: Some(InstanceType::String.into()),
                    string: Some(Box::new(StringValidation {
                        pattern: Some(concat!("0x\\w{", stringify!($len_in_chars),"}").to_string()),
                        ..Default::default()
                    })),
                    ..Default::default()
                }
                .into()
            }
        }
    };
}

schema_uint!(U8);
schema_uint!(U16);
schema_uint!(U24);
schema_uint!(U32);
schema_uint!(U40);
schema_uint!(U48);
schema_uint!(U56);
schema_uint!(U64);
schema_uint!(U72);
schema_uint!(U80);
schema_uint!(U88);
schema_uint!(U96);
schema_uint!(U104);
schema_uint!(U112);
schema_uint!(U120);
schema_uint!(U128);
schema_uint!(U136);
schema_uint!(U144);
schema_uint!(U152);
schema_uint!(U160);
schema_uint!(U168);
schema_uint!(U176);
schema_uint!(U184);
schema_uint!(U192);
schema_uint!(U200);
schema_uint!(U208);
schema_uint!(U216);
schema_uint!(U224);
schema_uint!(U232);
schema_uint!(U240);
schema_uint!(U248);
schema_uint!(U256);


schema_uint!(I8);
schema_uint!(I16);
schema_uint!(I24);
schema_uint!(I32);
schema_uint!(I40);
schema_uint!(I48);
schema_uint!(I56);
schema_uint!(I64);
schema_uint!(I72);
schema_uint!(I80);
schema_uint!(I88);
schema_uint!(I96);
schema_uint!(I104);
schema_uint!(I112);
schema_uint!(I120);
schema_uint!(I128);
schema_uint!(I136);
schema_uint!(I144);
schema_uint!(I152);
schema_uint!(I160);
schema_uint!(I168);
schema_uint!(I176);
schema_uint!(I184);
schema_uint!(I192);
schema_uint!(I200);
schema_uint!(I208);
schema_uint!(I216);
schema_uint!(I224);
schema_uint!(I232);
schema_uint!(I240);
schema_uint!(I248);
schema_uint!(I256);

schema_fixed_hash!(B8, 2);
schema_fixed_hash!(B16, 4);
schema_fixed_hash!(B32, 8);
schema_fixed_hash!(B64, 16);
schema_fixed_hash!(B96, 24);
schema_fixed_hash!(B128, 32);
schema_fixed_hash!(B160, 40);
schema_fixed_hash!(B192, 48);
schema_fixed_hash!(B224, 56);
schema_fixed_hash!(B256, 64);

schema_fixed_hash!(Address, 40);
