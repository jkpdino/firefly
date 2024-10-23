# Test Framework

The Firefly test framework uses source code annotated by comments indicating the behavior it wants.

A test, or a folder of tests can be run.

Comments include

/** @include file.fly **/

Includes the file relative to the current directory

/** @notest **/

Don't run this as a test

/** @error ...? **/

Expect an error at the current location

/** @expect ...? **/
Expect the line outputted

## Display Engine

