const ffi = require('ffi-napi');
//                  ~~~~~~~~~~
//                  Node.js Foreign Function Interface for N-API
//                  https://www.npmjs.com/package/ffi-napi

const os = require('os');
const path = require('path');

// Dynamic library filename change depending on the Operating System (OS) ...
let filename;
if (os.platform() === 'win32') {
  filename = 'libsnip.dll';
  //          ~~~~~~~.dll on Windows
} else if (os.platform() === 'linux') {
  filename = 'libsnip.so';
  //          ~~~~~~~.so on Linux
} else if (os.platform() === 'darwin') {
  filename = 'libsnip.dylib';
   //         ~~~~~~~.dylib on macOS
}

exports.snip = ffi.Library(path.join(__dirname, 'target/debug', filename),
  //                       ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  //                       Path to the dynamic library loaded by Node.js
  //                         e.g. target/debug/libsnip.dll on Windows
  {
    'add': ['int', ['int', 'int']],
    //     ~~~~~~~~~~~~~~~~~~~~~~~
    //     Type signature of the function loaded, as:
    //
    //         function_name: [return_value, [ ...arguments ]]

    // Other example: 'concat': ['String', ['String', 'String']],
  }
);
