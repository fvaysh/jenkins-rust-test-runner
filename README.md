# jenkins-rust-test-runner


A **Rust-based test execution framework** that integrates with **Jenkins CI/CD** for **automated testing** and structured reporting. This tool enables **isolated, reliable test execution** using **Docker**, **JUnit XML reports**, and parallel test runs.

##  Features

- **Automates Rust tests (`cargo test`)** in a Jenkins pipeline  
- **Jenkinsfile** defines structured test execution  
- **Parallel test execution** using `rayon` for efficiency  
- **Generates structured JUnit XML reports** for Jenkins visualization  
- **Runs tests inside Docker** for consistency across environments  
- **Log processing module** captures warnings/errors from test logs  

##  Logs

[2025-03-23 12:00:01] INFO: Starting test execution...
[2025-03-23 12:00:02] INFO: Running test 'test_filter_errors'...
[2025-03-23 12:00:02] PASS: test_filter_errors 

[2025-03-23 12:00:03] INFO: Running test 'test_count_warnings'...
[2025-03-23 12:00:03] PASS: test_count_warnings 

[2025-03-23 12:00:04] INFO: Running test 'test_parallel_execution'...
[2025-03-23 12:00:05] WARNING: Parallel task 2 took longer than expected.
[2025-03-23 12:00:06] PASS: test_parallel_execution 

[2025-03-23 12:00:07] INFO: Running test 'test_json_to_csv'...
[2025-03-23 12:00:07] ERROR: Mismatch in expected CSV format.
[2025-03-23 12:00:07] FAIL: test_json_to_csv 

[2025-03-23 12:00:08] INFO: Running test 'test_csv_to_json'...
[2025-03-23 12:00:08] PASS: test_csv_to_json 

[2025-03-23 12:00:09] INFO: Test execution completed.
[2025-03-23 12:00:09] SUMMARY:
[2025-03-23 12:00:09] - Passed: 4 
[2025-03-23 12:00:09] - Failed: 1 
[2025-03-23 12:00:09] - Warnings: 1 






