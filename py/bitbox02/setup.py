# Copyright 2019 Shift Cryptosecurity AG
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
"""BitBox python package"""
from distutils.core import setup
import setuptools

with open("README.md", "r") as fh:
    LONG_DESCRIPTION = fh.read()

setup(
    name="bitbox02",
    version="1.0.0",
    author="Shift Cryptosecurity",
    author_email="support@shiftcrypto.ch",
    packages=setuptools.find_packages(),
    description="Python library for bitbox02 communication",
    long_description=LONG_DESCRIPTION,
    long_description_content_type="text/markdown",
    url="https://github.com/digitalbitbox/bitbox02-firmware",
    python_requires=">=3.6",
    classifiers=[
        "Intended Audience :: Developers",
        "License :: OSI Approved :: Apache Software License",
        "Programming Language :: Python :: 3.6",
    ],
    keywords="digitalbitbox bitbox bitbox02 bitcoin litecoin ethereum erc20 u2f",
    install_requires=[
        "hidapi>=0.7.99.post21,<0.8",
        "noiseprotocol>=0.3,<0.4",
        "protobuf>=3.7,<3.8",
        "ecdsa>=0.13,<0.14",
        "semver>=2.8.1,<3.0",
        # Needed as long as we support python < 3.7
        "typing>=3.7.4,<3.8",
        "typing_extensions>=3.7.4,<3.8",
    ],
)
