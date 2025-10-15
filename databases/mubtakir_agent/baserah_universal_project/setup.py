#!/usr/bin/env python3
"""
Setup script for Baserah Universal
🧬 Creator: Basil Yahya Abdullah
🌟 Revolutionary Mathematical Intelligence System
"""

from setuptools import setup, find_packages

with open("README.md", "r", encoding="utf-8") as fh:
    long_description = fh.read()

with open("requirements.txt", "r", encoding="utf-8") as fh:
    requirements = [line.strip() for line in fh if line.strip() and not line.startswith("#")]

setup(
    name="baserah-universal",
    version="1.0.0",
    author="Basil Yahya Abdullah",
    author_email="",
    description="Revolutionary Mathematical Intelligence - Natural Language to Mathematical Equations",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://huggingface.co/spaces/Mubtakir/baserah-universal",
    packages=find_packages(),
    classifiers=[
        "Development Status :: 4 - Beta",
        "Intended Audience :: Developers",
        "Intended Audience :: Science/Research",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Topic :: Scientific/Engineering :: Mathematics",
        "Topic :: Scientific/Engineering :: Artificial Intelligence",
        "Topic :: Text Processing :: Linguistic",
        "Natural Language :: Arabic",
    ],
    python_requires=">=3.8",
    install_requires=requirements,
    keywords=[
        "mathematical-intelligence",
        "natural-language-processing", 
        "arabic-language",
        "mathematical-equations",
        "shape-generation",
        "revolutionary-ai",
        "transparent-ai",
        "no-neural-networks"
    ],
    project_urls={
        "Bug Reports": "https://github.com/basilyahya/baserah-universal/issues",
        "Source": "https://github.com/basilyahya/baserah-universal",
        "Demo": "https://huggingface.co/spaces/Mubtakir/baserah-universal",
    },
)
