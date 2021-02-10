# Install Dlib

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