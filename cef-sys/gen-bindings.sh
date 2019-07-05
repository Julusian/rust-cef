#!/bin/bash

bindgen vendor/cef/include/capi/cef_app_capi.h -o src/bindings.rs -- -I vendor/cef
