#!/bin/bash

SDL_V='2.0.22'
SDL_TTF_V='2.20.0'
SDL_IMG_V='2.6.0'

# msvc
sdl_msvc="https://github.com/libsdl-org/SDL/releases/download/release-${SDL_V}/SDL2-devel-${SDL_V}-VC.zip"
img_msvc="https://github.com/libsdl-org/SDL_image/releases/download/release-${SDL_IMG_V}/SDL2_image-devel-${SDL_IMG_V}-VC.zip"
ttf_msvc="https://github.com/libsdl-org/SDL_ttf/releases/download/release-${SDL_TTF_V}/SDL2_ttf-devel-${SDL_TTF_V}-VC.zip"

# mingw
sdl_mingw="https://github.com/libsdl-org/SDL/releases/download/release-${SDL_V}/SDL2-devel-${SDL_V}-mingw.zip"
img_mingw="https://github.com/libsdl-org/SDL_image/releases/download/release-${SDL_IMG_V}/SDL2_image-devel-${SDL_IMG_V}-mingw.zip"
ttf_mingw="https://github.com/libsdl-org/SDL_ttf/releases/download/release-${SDL_TTF_V}/SDL2_ttf-devel-${SDL_TTF_V}-mingw.zip"

echo $sdl_mingw
echo $img_mingw
echo $ttf_mingw

echo $sdl_msvc
echo $img_msvc
echo $ttf_msvc

cd ci

curl -o sdl_msvc.zip -L $sdl_msvc
curl -o sdl_img_msvc.zip -L $img_msvc
curl -o sdl_ttf_msvc.zip -L $ttf_msvc

curl -o sdl_mingw.zip -L $sdl_mingw
curl -o sdl_img_mingw.zip -L $img_mingw
curl -o sdl_ttf_mingw.zip -L $ttf_mingw

unzip sdl_mingw.zip -d sdl-mingw
unzip sdl_msvc.zip -d sdl-msvc

unzip sdl_img_mingw.zip -d img-mingw
unzip sdl_img_msvc.zip -d img-msvc

unzip sdl_ttf_mingw.zip -d ttf-mingw
unzip sdl_ttf_msvc.zip -d ttf-msvc

ls

mkdir -p gnu-mingw/dll/32
mkdir -p gnu-mingw/dll/64

mkdir -p gnu-mingw/lib/32
mkdir -p gnu-mingw/lib/64

mkdir -p msvc/dll/32
mkdir -p msvc/dll/64

mkdir -p msvc/lib/32
mkdir -p msvc/lib/64

# SDL2-2.0.22/x86_64-w64-mingw32/
# SDL2-2.0.22/include/
mv sdl-mingw/SDL2-${SDL_V}/i686-w64-mingw32/bin gnu-mingw/dll/32
mv sdl-mingw/SDL2-${SDL_V}/x86_64-w64-mingw32/bin gnu-mingw/dll/64
mv sdl-mingw/SDL2-${SDL_V}/i686-w64-mingw32/lib gnu-mingw/lib/32
mv sdl-mingw/SDL2-${SDL_V}/x86_64-w64-mingw32/lib gnu-mingw/lib/64
mv sdl-msvc/SDL2-${SDL_V}/lib/x86/*.dll msvc/dll/32
mv sdl-msvc/SDL2-${SDL_V}/lib/x64/*.dll msvc/dll/64
mv sdl-msvc/SDL2-${SDL_V}/lib/x86/*.lib msvc/lib/32
mv sdl-msvc/SDL2-${SDL_V}/lib/x64/*.lib msvc/lib/64

# image
# img-mingw/SDL2_image-2.6.0/i686-w64-mingw32/lib/
# img-msvc/SDL2_image-2.6.0/lib/
mv img-mingw/SDL2_image-${SDL_IMG_V}/i686-w64-mingw32/bin gnu-mingw/dll/32
mv img-mingw/SDL2_image-${SDL_IMG_V}/x86_64-w64-mingw32/bin gnu-mingw/dll/64
mv img-mingw/SDL2_image-${SDL_IMG_V}/i686-w64-mingw32/lib gnu-mingw/lib/32
mv img-mingw/SDL2_image-${SDL_IMG_V}/x86_64-w64-mingw32/lib gnu-mingw/lib/64
mv img-msvc/SDL2_image-${SDL_IMG_V}/lib/x86/*.dll msvc/dll/32
mv img-msvc/SDL2_image-${SDL_IMG_V}/lib/x64/*.dll msvc/dll/64
mv img-msvc/SDL2_image-${SDL_IMG_V}/lib/x86/*.lib msvc/lib/32
mv img-msvc/SDL2_image-${SDL_IMG_V}/lib/x64/*.lib msvc/lib/64

# ttf
# ttf-mingw/SDL2_ttf-2.20.0/x86_64-w64-mingw32/include
# ttf-msvc/SDL2_ttf-2.20.0/lib/
mv ttf-mingw/SDL2_ttf-${SDL_TTF_V}/i686-w64-mingw32/bin gnu-mingw/dll/32
mv ttf-mingw/SDL2_ttf-${SDL_TTF_V}/x86_64-w64-mingw32/bin gnu-mingw/dll/64
mv ttf-mingw/SDL2_ttf-${SDL_TTF_V}/i686-w64-mingw32/lib gnu-mingw/lib/32
mv ttf-mingw/SDL2_ttf-${SDL_TTF_V}/x86_64-w64-mingw32/lib gnu-mingw/lib/64
mv ttf-msvc/SDL2_ttf-${SDL_TTF_V}/lib/x86/*.dll msvc/dll/32
mv ttf-msvc/SDL2_ttf-${SDL_TTF_V}/lib/x64/*.dll msvc/dll/64
mv ttf-msvc/SDL2_ttf-${SDL_TTF_V}/lib/x86/*.lib msvc/lib/32
mv ttf-msvc/SDL2_ttf-${SDL_TTF_V}/lib/x64/*.lib msvc/lib/64
