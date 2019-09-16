#! /bin/bash
noteVal=12

for octave in {0..10} 
do
	for note in C CS D DS E F FS G GS A AS B 
	do
		echo "pub const $note$octave: f32=${noteVal}_f32;"
		noteVal=$((noteVal+1))
	done
done




