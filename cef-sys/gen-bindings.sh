#!/bin/bash

bindgen bindings.h -o src/bindings.rs -- -I vendor/cef
