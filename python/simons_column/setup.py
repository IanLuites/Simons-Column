from setuptools import setup, find_packages

setup(
    name="simons_column",
    version="0.1.0",
    description="A private package for Simon's Column",
    author="Ian Luites",
    author_email="Ian Luites",
    packages=find_packages(),
    package_data={
        "simons_column": ["lib/*.so", "lib/*.dylib", "lib/*.dll"],
    },
    include_package_data=True,
    install_requires=[],
    zip_safe=False,
    python_requires=">=3.6",
)
