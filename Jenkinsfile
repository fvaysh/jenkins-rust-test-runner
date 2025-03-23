pipeline {
    agent any
    environment {
        RUST_BACKTRACE = "1"
    }
    stages {
        stage('Checkout') {
            steps {
                git 'https://github.com/your-repo.git'
            }
        }
        stage('Setup') {
            steps {
                sh 'bash scripts/setup_env.sh'
            }
        }
        stage('Build') {
            steps {
                sh 'cargo build --verbose'
            }
        }
        stage('Run Tests') {
            steps {
                sh 'bash scripts/run_tests.sh'
            }
        }
        stage('Collect Code Coverage') {
            steps {
                sh 'cargo test -- --nocapture'
            }
        }
        stage('Publish Reports') {
            steps {
                junit 'artifacts/reports/test-results.xml'
                archiveArtifacts artifacts: 'artifacts/**', fingerprint: true
            }
        }
    }
}
