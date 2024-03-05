from scalecodec.base import RuntimeConfiguration
from scalecodec.type_registry import load_type_registry_preset

type_registry = {
    "types": {
        "ControllerArgs": {
            "type": "struct",
            "type_mapping": [
                ["storage", "AccountId"],
                ["test", "TestStruct"],
            ],
        },
        "TestStruct": {
            "type": "struct",
            "type_mapping": [
                [
                    "test_id",
                    "Vec<u8>",
                ],
                [
                    "az_address",
                    "AccountId",
                ],
                [
                    "az_o_address",
                    "Option<AccountId>",
                ],
                [
                    "eth_address",
                    "[u8; 20]",
                ],
                [
                    "eth_o_address",
                    "Option<[u8; 20]>",
                ],
            ]
        },
    }
}

conf = RuntimeConfiguration()
conf.update_type_registry(load_type_registry_preset("core"))
conf.update_type_registry(load_type_registry_preset("legacy"))
conf.update_type_registry(type_registry)
struct_obj = conf.create_scale_object("TestStruct")
args_obj = conf.create_scale_object("ControllerArgs")

test_id = "122-122-124"
test_id_vec = [ord(_) for _ in test_id]
print(f"Test id: {test_id}")

test_struct = {
    "test_id": test_id_vec,
    "az_address": "5GMrxGBPvjAPWmjEJ3duJymbuRyrCj7mLax83ntnUBbqALkL",
    # "az_o_address": None,
    "az_o_address": "5GMrxGBPvjAPWmjEJ3duJymbuRyrCj7mLax83ntnUBbqALkL",
    "eth_address": "0xF7DE62B65768a169279be74b12FaA65a22FB38D3",
    "eth_o_address": "0xF7DE62B65768a169279be74b12FaA65a22FB38D3",
}
scale_struct = struct_obj.encode(test_struct)
print(f"Scale struct: 0x{scale_struct.data.hex()}")

test_args = {
    "storage": "5FqFKpdLSU9iw61U1gbQFXM2V312DXfi2rUc39RYsqAgnJX6",
    "test": test_struct,
}
scale_args = args_obj.encode(test_args)
print(f"Scale args: 0x{scale_args.data.hex()}")
