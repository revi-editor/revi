// extern crate revi_core;
//
// use revi_core::Buffer;
//
// #[test]
// fn test_next_jump_idx() {
//     // TODO: Read in test.txt file and make good test.
//     //                                 11111111112                                  |
//     //                       012345678901234567890                                  |
//     //                       🠧     🠧 🠧                                              |
//     let string = "Foo(2.3);\nHello 1 ------- :: WOW\nlet something = foo();\n";
//     let buffer = Buffer::new_str(string);
//     let line_idx = buffer.next_jump_idx(&(0, 1).into());
//     assert_eq!(line_idx, Some(6));
//     let line_idx = buffer.next_jump_idx(&(line_idx.unwrap(), 1).into());
//     assert_eq!(line_idx, Some(8));
// }
