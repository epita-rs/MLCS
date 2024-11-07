# A bash testsuite for epita.rs MLCS project

## Run the testsuite
At the root of the repo, run the following command
```
tests/./testsuite.sh
```
## Add your own tests 
1. Add text files to the folder testfiles/
2. Compare the added files <br>
Let's suppose you added f1 f2 and f3. <br>
Now search for the testsuite function in test/testsuite.sh. <br>
Finally append the following function call to the function
```
tes f1 f2 f3
```
