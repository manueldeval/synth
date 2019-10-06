ps -ef | grep rustic_synth | grep -v grep | xargs | cut -f2 -d' ' | xargs kill -9
