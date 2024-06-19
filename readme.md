# BSP parsing library

> [!WARNING]
> Work in progress

This library aims to be a platform agnostic, and target agnostic decoder and
loader of BSP (Binary space partitioning) files.

Current target support: HL BSP ver3 (GoldSrc format).

> [!CAUTION]
> Currently the node parsing is not ready (Broken and failing in most
> tests).

## Roadmap

- [ ] Implement missing features (Some are broken placeholders like VIS and
      Nodes)
- [x] Parse meshes
- [x] Parse entities
- [ ] :star: Create a reader that compiles all brushes and gives one by one to
      an iterator or a collection.
