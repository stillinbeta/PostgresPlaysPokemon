import setuptools

with open('../README.md', 'r') as rm:
    long_description = rm.read()
    
setuptools.setup(
    name = "paas",
    version="0.1.0",
    author="stillinbeta",
    author_email="web@stillinbeta.com",
    long_description=long_description,
    url="https://github.com/stillinbeta/PostgresPlaysPokemon",
    packages=setuptools.find_packages(),
    classifiers = [
        "Programming Language :: Python :: Implementation :: PyPy",
        "Topic :: System :: Emulators",
    ],
    install_requires=[
        "PyBoy",
        "grpcio==1.18.0",
        "grpcio-tools==1.18.0",
        "protobuf==3.6.1",
    ],
    dependency_links=[
        "https://github.com/Baekalfen/PyBoy/archive/7d5ed850d4b9dd03cd4a20fc8acf75f938399c1f.zip#egg=PyBoy&subdirectory=Source"
    ],
)
