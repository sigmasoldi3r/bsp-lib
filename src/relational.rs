use crate::{
    bsp::Bsp,
    lumps::{faces::BspFace, vertices::BspVertex},
};

// fn range_from<A, B>(from: &A, to: &B) -> Option<Range<usize>> {
//   let from: usize = from.try_into()?;
//   let to: usize = to.try_into()?;
//   Some(from..to)
// }

impl BspFace {
    pub fn vertices<'a>(&self, bsp: &'a Bsp) -> Vec<&'a BspVertex> {
        let i_first_edge = self.i_first_edge as usize;
        let n_edges = (self.i_first_edge + self.n_edges as u32) as usize;
        let i_last_edge = i_first_edge + n_edges;
        let i_surf_edges = &bsp.surf_edges.0[i_first_edge..i_last_edge];
        let mut vertices: Vec<&'a BspVertex> = vec![];
        for i_surf_edge in i_surf_edges {
            let index = i_surf_edge.0;
            let i_vertex = if index < 0 {
                let edge = &bsp.edges[-index as usize];
                edge.i_vertex[1]
            } else {
                let edge = &bsp.edges[index as usize];
                edge.i_vertex[0]
            };
            let vertex = &bsp.vertices[i_vertex as usize];
            vertices.push(vertex);
        }
        vertices
    }
}
