pub mod schematic;
pub mod verifier;

#[cfg(test)]
mod tests {
    use super::schematic::{BlockState, SchematicGrid};
    use super::verifier::{
        evaluate_block, find_item_slot_in_inventory, verify_layer, BlockMatchStatus,
    };
    use std::collections::HashMap;

    #[test]
    fn test_observer_and_piston_rotation() {
        let mut props = HashMap::new();
        props.insert("facing".to_string(), "north".to_string());
        props.insert("powered".to_string(), "false".to_string());

        let observer = BlockState::new("minecraft:observer".to_string(), props);
        let rot90 = observer.rotated(1);
        let rot180 = observer.rotated(2);
        let rot270 = observer.rotated(3);

        assert!(rot90.properties.contains(&("facing".into(), "east".into())));
        assert!(rot180.properties.contains(&("facing".into(), "south".into())));
        assert!(rot270.properties.contains(&("facing".into(), "west".into())));
    }

    #[test]
    fn test_bit_array_unpacking() {
        // 2 bits por entrada, 4 blocos: índices [0, 1, 2, 3] -> binário 11_10_01_00 = 0xE4
        let packed = vec![0xE4i64];
        let unpacked = SchematicGrid::unpack_litematic_bit_array(&packed, 2, 4);
        assert_eq!(unpacked, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_inventory_search_beyond_hotbar() {
        let mut inv: Vec<Option<String>> = vec![None; 36];
        inv[35] = Some("minecraft:observer".to_string());

        let slot = find_item_slot_in_inventory(&inv, "minecraft:observer");
        assert_eq!(slot, Some(35));
    }

    #[test]
    fn test_verification_status() {
        let mut p1 = HashMap::new();
        p1.insert("facing".into(), "north".into());
        let expected = BlockState::new("minecraft:hopper".into(), p1);

        let mut p2 = HashMap::new();
        p2.insert("facing".into(), "south".into());
        let wrong_dir = BlockState::new("minecraft:hopper".into(), p2);

        assert_eq!(
            evaluate_block(&expected, &wrong_dir),
            BlockMatchStatus::WrongStateOrOrientation
        );

        let grid = SchematicGrid {
            width: 1,
            height: 1,
            length: 1,
            palette: vec![expected.clone()],
            blocks: vec![0],
        };

        let report = verify_layer(&grid, 0, 0, |_, _, _| expected.clone());
        assert_eq!(report.completion_percentage(), 100.0);
    }
}
