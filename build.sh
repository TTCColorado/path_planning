#!/bin/sh
docker run -v /home/tsturzl/projects/pathplanning-pybind:/opt/path-planning -it x86_64/pyo3-builder
docker run -v /home/tsturzl/projects/pathplanning-pybind:/opt/path-planning -it arm64v8/pyo3-builder
