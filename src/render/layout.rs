//  struct Page { Img }
//                ^^^
//                Bit/Anim
//  Layout
//  Canvas
//
//
// Img::Bit
//   loop {
//     read_file() -> Img;
//     ? gen_layout();
//     Img.resize(&mut bytes);
//     Page.flush(&mut Layout);
//     Layout.display();
//     Canvas.flush();
//   }
//
//
// Img::Anim
//   loop {
//     read_file() -> Img;
//     Img.to_next_frame().resize(&mut bytes);
//     Page.flush(&mut Layout);
//     Layout.display();
//     Canvas.flush();
//   }
//

use crate::*;

// ? Layout( Pages )
//   vs
//   Pages { Layout }
//
// self.match {
//   xxx => { pages.layout_x() }
//   yyy => { pages.layout_y() }
// }
//
// layout.from(InitMode {}).display()

pub enum Layout {
    //  +-------------------+ [cur page]
    //  |                   |
    //  |                   |
    //  |        P1         |
    //  |                   |
    //  |                   |
    //  +-------------------+ [next page]
    //  | .  .  .  .  .  .  |
    //  | .  .  .  .  .  .  |
    //  | .  .   P2   .  .  |
    //  +-------------------+
    //         page{1}
    Single,

    //  +---------+---------+ [cur page]
    //  |         |         |
    //  |         |         |
    //  |   P1    |   P2    |
    //  |         |         |
    //  |         |         |
    //  +---------+---------+ [next page]
    //  | .  .  . | .  .  . |
    //  | .  .  . | .  .  . |
    //  | . P1  . | . P2  . |
    //  +---------+---------+
    //        page{1, 2}
    Double,

    //  <Scroll Only>
    //               (right to left)
    //  +----+----+----+----+
    //  |    |    |    |    |
    //  |    |    |    |    |
    //  | P1 | P2 | P3 | .. |
    //  |    |    |    |    |
    //  |    |    |    |    |
    //  +----+----+----+----+
    // (left to right)
    //       page{1, 2, ..}
    Multi,

    //  <Scroll + thumb only>
    //                 (up to down)
    //  +---------+---------+
    //  |         |         |
    //  |         |   P2    |
    //  |   P1    |         |
    //  |         +---------+
    //  |         |         |
    //  +---------+         |
    //  |         |         |
    //  |   P3    |   P4    |
    //  |         |         |
    //  +---------+         |
    //  |         |         |
    //  |   P5    +---------+
    //  |         |   ..    |
    //  +---------+---------+
    //        page{1, 2}
    //
    Masonry,
}

impl Layout {
    fn resize_masonry(&mut self, data: &Data) -> anyhow::Result<()> {
        // REFS: https://www.google.com/url?q=https://stackoverflow.com/questions/68832902/qml-staggered-masonry-gridview-or-flow&sa=U&ved=2ahUKEwjjrt_Y_J-CAxUhM0QIHWjQAowQFnoECAoQAg&usg=AOvVaw3FEnTdXmJTcFoJuTaueOu9
        // TODO:
        //
        Ok(())
    }
}

// GPT:
//fn display_masonry_layout(image_list: Vec<&[u32]>) -> &[u32] {
//    // Create a vector of image-height pairs
//    let mut image_heights: Vec<(usize, usize)> = image_list
//        .iter()
//        .map(|image| (image.len(), image.len() / 4))
//        .enumerate()
//        .collect();
//
//    // Sort the image-height pairs in descending order of height
//    image_heights.sort_by_key(|&(_, height)| std::cmp::Reverse(height));
//
//    // Create a vector of columns, initialized with empty vectors
//    let num_columns = 3;
//    let mut columns: Vec<Vec<usize>> = vec![Vec::new(); num_columns];
//
//    // Add each image to the shortest column
//    for (image_index, (_, height)) in image_heights {
//        let shortest_column_index = columns
//            .iter()
//            .enumerate()
//            .min_by_key(|&(_, column)| column.iter().map(|&i| image_heights[i].1).sum::<usize>())
//            .unwrap()
//            .0;
//        columns[shortest_column_index].push(image_index);
//    }
//
//    // Concatenate the columns into a single vector of image indices
//    let mut layout: Vec<usize> = Vec::new();
//    for row_index in 0..(image_list.len() / num_columns + 1) {
//        for column_index in 0..num_columns {
//            if let Some(image_index) = columns.get(column_index)?.get(row_index) {
//                layout.push(*image_index);
//            }
//        }
//    }
//
//    // Convert the vector of image indices to a slice of u32 values
//    let layout_slice: &[u32] = layout
//        .iter()
//        .flat_map(|&image_index| image_list.get(image_index).unwrap_or(&[]).iter())
//        .copied()
//        .collect::<Vec<u32>>()
//        .as_slice();
//
//    layout_slice
//}
