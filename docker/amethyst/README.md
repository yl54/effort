## Summary

Amethyst runs its CICD on Jenkins. In order to make sure that certain things will work on local and CICD, it is good to know how to manually run tests that would be on CICD on local. The Docker world will not always mirror your local machine, so its important to test both.

## How to manually build a amethyst docker container

* Create the base image for the container
* Build the container
* Run the container
* Copy your files into the container
* Run your unit tests.
