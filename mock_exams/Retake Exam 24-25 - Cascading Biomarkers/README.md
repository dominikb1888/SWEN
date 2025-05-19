## Cascading Biomarkers

You are part of a medical research team analyzing patient health data to predict future trends. The data consists of a series of measurements for various biomarkers (e.g., blood pressure, heart rate, cholesterol levels) collected over time. For simplicity, the biomarkers are represented as sequences of integers in a report.

Your task is to predict the next value for each biomarker sequence using a method based on the difference between consecutive measurements. The prediction algorithm is as follows:

Calculate the first-level differences between consecutive measurements.

Repeat this process on the differences until you reach a sequence where all values are zero.

Once you have an all-zero difference sequence, you can extrapolate the next value for the original sequence by:

Adding a new zero at the end of the zero-difference sequence.

Propagating the values back up the difference layers.

For example, given the biomarker sequence 120 123 126 129 132 135 (representing blood pressure readings):

First-level differences: 3 3 3 3 3
Second-level differences: 0 0 0 0

The prediction is 138.

Your task is to predict the next value for each biomarker and compute the sum of these predictions.

Please submit ONE lib.rs file only. Do NOT submit more files or archives please.
