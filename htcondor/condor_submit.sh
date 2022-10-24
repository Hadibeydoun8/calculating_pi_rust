#! /bin/sh
# This script is used to submit a job to the HTCondor batch system.
# It is called by the condor_submit command.
# It is not intended to be called directly.

# First Build the Calculating Pi Executable
cd ../
git reset --hard
git pull
cargo build --release

# Stage the Executable
cp target/release/calculating_pi_rust ./htcondor/calculating_pi_rust

# Reset and Create the Output Directories
cd htcondor || exit
rm -rf output error log
mkdir output error log

# Then Submit the Job
condor_submit calculating_pi.submit

# Finally, Check the Job Status
watch condor_q