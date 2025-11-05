# Install script for directory: /home/beth/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sdl3-image-src-3.2.4/SDL_image

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/home/beth/Code/rust/HoyoUMU/target/debug/build/sdl3-image-sys-2cd29e3a8730c177/out")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Debug")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Install shared libraries without execute permission?
if(NOT DEFINED CMAKE_INSTALL_SO_NO_EXE)
  set(CMAKE_INSTALL_SO_NO_EXE "0")
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

# Set path to fallback-tool for dependency-resolution.
if(NOT DEFINED CMAKE_OBJDUMP)
  set(CMAKE_OBJDUMP "/usr/bin/objdump")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "library" OR NOT CMAKE_INSTALL_COMPONENT)
  foreach(file
      "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib64/libSDL3_image.so.0.2.4"
      "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib64/libSDL3_image.so.0"
      )
    if(EXISTS "${file}" AND
       NOT IS_SYMLINK "${file}")
      file(RPATH_CHECK
           FILE "${file}"
           RPATH "")
    endif()
  endforeach()
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib64" TYPE SHARED_LIBRARY FILES
    "/home/beth/Code/rust/HoyoUMU/target/debug/build/sdl3-image-sys-2cd29e3a8730c177/out/build/libSDL3_image.so.0.2.4"
    "/home/beth/Code/rust/HoyoUMU/target/debug/build/sdl3-image-sys-2cd29e3a8730c177/out/build/libSDL3_image.so.0"
    )
  foreach(file
      "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib64/libSDL3_image.so.0.2.4"
      "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib64/libSDL3_image.so.0"
      )
    if(EXISTS "${file}" AND
       NOT IS_SYMLINK "${file}")
      file(RPATH_CHANGE
           FILE "${file}"
           OLD_RPATH "/home/beth/Code/rust/HoyoUMU/target/debug/build/sdl3-sys-fc9c14065e331635/out/lib64:"
           NEW_RPATH "")
      if(CMAKE_INSTALL_DO_STRIP)
        execute_process(COMMAND "/usr/bin/strip" "${file}")
      endif()
    endif()
  endforeach()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "library" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib64" TYPE SHARED_LIBRARY FILES "/home/beth/Code/rust/HoyoUMU/target/debug/build/sdl3-image-sys-2cd29e3a8730c177/out/build/libSDL3_image.so")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "DEVEL" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/SDL3_image" TYPE FILE FILES "/home/beth/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sdl3-image-src-3.2.4/SDL_image/include/SDL3_image/SDL_image.h")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "devel" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib64/cmake/SDL3_image" TYPE FILE FILES
    "/home/beth/Code/rust/HoyoUMU/target/debug/build/sdl3-image-sys-2cd29e3a8730c177/out/build/SDL3_imageConfig.cmake"
    "/home/beth/Code/rust/HoyoUMU/target/debug/build/sdl3-image-sys-2cd29e3a8730c177/out/build/SDL3_imageConfigVersion.cmake"
    )
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "devel" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib64/cmake/SDL3_image/SDL3_image-shared-targets.cmake")
    file(DIFFERENT _cmake_export_file_changed FILES
         "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib64/cmake/SDL3_image/SDL3_image-shared-targets.cmake"
         "/home/beth/Code/rust/HoyoUMU/target/debug/build/sdl3-image-sys-2cd29e3a8730c177/out/build/CMakeFiles/Export/b76b131a8258361e9b58114039e89610/SDL3_image-shared-targets.cmake")
    if(_cmake_export_file_changed)
      file(GLOB _cmake_old_config_files "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib64/cmake/SDL3_image/SDL3_image-shared-targets-*.cmake")
      if(_cmake_old_config_files)
        string(REPLACE ";" ", " _cmake_old_config_files_text "${_cmake_old_config_files}")
        message(STATUS "Old export file \"$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib64/cmake/SDL3_image/SDL3_image-shared-targets.cmake\" will be replaced.  Removing files [${_cmake_old_config_files_text}].")
        unset(_cmake_old_config_files_text)
        file(REMOVE ${_cmake_old_config_files})
      endif()
      unset(_cmake_old_config_files)
    endif()
    unset(_cmake_export_file_changed)
  endif()
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib64/cmake/SDL3_image" TYPE FILE FILES "/home/beth/Code/rust/HoyoUMU/target/debug/build/sdl3-image-sys-2cd29e3a8730c177/out/build/CMakeFiles/Export/b76b131a8258361e9b58114039e89610/SDL3_image-shared-targets.cmake")
  if(CMAKE_INSTALL_CONFIG_NAME MATCHES "^([Dd][Ee][Bb][Uu][Gg])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib64/cmake/SDL3_image" TYPE FILE FILES "/home/beth/Code/rust/HoyoUMU/target/debug/build/sdl3-image-sys-2cd29e3a8730c177/out/build/CMakeFiles/Export/b76b131a8258361e9b58114039e89610/SDL3_image-shared-targets-debug.cmake")
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "devel" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib64/pkgconfig" TYPE FILE FILES "/home/beth/Code/rust/HoyoUMU/target/debug/build/sdl3-image-sys-2cd29e3a8730c177/out/build/sdl3-image.pc")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "library" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/licenses/SDL3_image" TYPE FILE FILES "/home/beth/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sdl3-image-src-3.2.4/SDL_image/LICENSE.txt")
endif()

string(REPLACE ";" "\n" CMAKE_INSTALL_MANIFEST_CONTENT
       "${CMAKE_INSTALL_MANIFEST_FILES}")
if(CMAKE_INSTALL_LOCAL_ONLY)
  file(WRITE "/home/beth/Code/rust/HoyoUMU/target/debug/build/sdl3-image-sys-2cd29e3a8730c177/out/build/install_local_manifest.txt"
     "${CMAKE_INSTALL_MANIFEST_CONTENT}")
endif()
if(CMAKE_INSTALL_COMPONENT)
  if(CMAKE_INSTALL_COMPONENT MATCHES "^[a-zA-Z0-9_.+-]+$")
    set(CMAKE_INSTALL_MANIFEST "install_manifest_${CMAKE_INSTALL_COMPONENT}.txt")
  else()
    string(MD5 CMAKE_INST_COMP_HASH "${CMAKE_INSTALL_COMPONENT}")
    set(CMAKE_INSTALL_MANIFEST "install_manifest_${CMAKE_INST_COMP_HASH}.txt")
    unset(CMAKE_INST_COMP_HASH)
  endif()
else()
  set(CMAKE_INSTALL_MANIFEST "install_manifest.txt")
endif()

if(NOT CMAKE_INSTALL_LOCAL_ONLY)
  file(WRITE "/home/beth/Code/rust/HoyoUMU/target/debug/build/sdl3-image-sys-2cd29e3a8730c177/out/build/${CMAKE_INSTALL_MANIFEST}"
     "${CMAKE_INSTALL_MANIFEST_CONTENT}")
endif()
