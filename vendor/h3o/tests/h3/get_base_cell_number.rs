use super::h3api;
use h3o::CellIndex;

macro_rules! test {
    ($name:ident, $index:literal) => {
        #[test]
        fn $name() {
            let index = CellIndex::try_from($index).expect("cell index");
            let result = index.base_cell();
            let reference = h3api::get_base_cell_number(index);

            assert_eq!(result, reference);
        }
    };
}

test!(hexagon, 0x87283080dffffff);
test!(pentagon, 0x870800000ffffff);
