use crate::{bsp::Bsp, lumps::{faces::BspFace, surfaces::BspEdge, vertices::BspVertex}};


impl BspFace {
  pub fn edges<'a>(&self, bsp: &'a Bsp) -> &'a [BspEdge] {
    let from = self.i_first_edge as usize;
    let to = (self.i_first_edge+self.n_edges as u32) as usize;
    &bsp.edges.0[from..to]
  }
}

impl BspEdge {
  pub fn vertices<'a>(&self, bsp: &'a Bsp) -> (&'a BspVertex, &'a BspVertex) {
    let from = self.i_vertex[0] as usize;
    let to = self.i_vertex[1] as usize;
    (&bsp.vertices.0[from], &bsp.vertices.0[to])
  }
}
