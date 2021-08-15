# HomeSec

This is a _very_ poorly organized size project of mine. The code contained is mostly various attempts to figure out how to use a Rust OpenCV implementation. The goal is to detect faces as they appear in my home security feed, collect and identify them. Once an identified face shows up in my feed, I will receive a SMS notification.

### Setup Notes

Install Dlib

```sh
wget http://dlib.net/files/dlib-19.21.tar.bz2
tar xvf dlib-19.21.tar.bz2
cd dlib-19.21/
mkdir build
cd build
cmake ..
cmake --build . --config Release
sudo make install
sudo ldconfig
cd ..
```


sudo apt-get install libblas-dev liblapack-dev
sudo apt-get install libgsl-dev
sudo apt-get install libatlas-base-dev 
