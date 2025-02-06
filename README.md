# xacro
A xml preprocessor for xacro files to generate URDF files

## WIP
Placeholder for the future xacro peprocessing tool. 
Reference: https://github.com/ros/xacro/wiki

## TODO
The following functionality shall be implemented:
- [ ] macro
- [ ] include
- [ ] insert_block
- [ ] property
- [ ] element
- [ ] if
- [ ] unless
- [ ] loop

These seem like the core functionalities required for proper xacro file handling.

The following might be implemented:
- [ ] load_yaml, unsure if yaml & xacro split is the right way to do things. Why not have a xacro with the properties?
- [ ] eval-comments, not sure how useful this will be vs the amount of effort required to implement this.

Fundamentally these feel like scope creep or nice-to-haves

The following shall not be implemented:
- [ ] Python-based evaluation
- [ ] rospack-based evaluation
- [ ] Windows compatibility

Because this package is meant to be as dependency free as possible, so no python or ros dependencies.
