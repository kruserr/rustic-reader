### [<-](../README.md)

## Benchmark
Quick 3 run pdftotext version 24.02.0 benchmark on Intel i5-8250U
```log
$ time pdftotext test-data/pdf/pdfreference1.7old.pdf - > test-data/txt/pdftotext-24.02.0-pdfreference1.7old.pdf.txt

________________________________________________________
Executed in    5.40 secs    fish           external
   usr time    4.96 secs    0.77 millis    4.96 secs
   sys time    0.43 secs    3.81 millis    0.43 secs

$ time pdftotext test-data/pdf/pdfreference1.7old.pdf - > test-data/txt/pdftotext-24.02.0-pdfreference1.7old.pdf.txt

________________________________________________________
Executed in    5.57 secs    fish           external
   usr time    5.16 secs    1.12 millis    5.16 secs
   sys time    0.40 secs    7.42 millis    0.39 secs

$ time pdftotext test-data/pdf/pdfreference1.7old.pdf - > test-data/txt/pdftotext-24.02.0-pdfreference1.7old.pdf.txt

________________________________________________________
Executed in    5.73 secs    fish           external
   usr time    5.27 secs    0.03 millis    5.27 secs
   sys time    0.46 secs    3.45 millis    0.45 secs

```

Quick 3 run cli-pdf-to-text version 0.1.1 benchmark on Intel i5-8250U
```log
$ time cli-pdf-to-text test-data/pdf/pdfreference1.7old.pdf - > test-data/txt/cli-pdf-to-text-0.1.1-pdfreference1.7old.pdf.txt

________________________________________________________
Executed in   14.06 secs    fish           external
   usr time   16.20 secs    1.64 millis   16.20 secs
   sys time    0.87 secs    0.00 millis    0.87 secs

$ time cli-pdf-to-text test-data/pdf/pdfreference1.7old.pdf - > test-data/txt/cli-pdf-to-text-0.1.1-pdfreference1.7old.pdf.txt

________________________________________________________
Executed in   13.69 secs    fish           external
   usr time   15.75 secs    2.40 millis   15.75 secs
   sys time    0.93 secs    1.05 millis    0.93 secs

$ time cli-pdf-to-text test-data/pdf/pdfreference1.7old.pdf - > test-data/txt/cli-pdf-to-text-0.1.1-pdfreference1.7old.pdf.txt

________________________________________________________
Executed in   13.72 secs    fish           external
   usr time   15.88 secs    0.00 millis   15.88 secs
   sys time    0.84 secs    6.37 millis    0.84 secs

```

The performance on the cli-pdf-to-text pdf converter is not as good as the pdftotext pdf converter, here is room for improvement.

The cli-pdf-to-text version 0.1.1 spits out a row of conversion errors, about 7630 lines, this needs to be fixed.

Also the output of cli-pdf-to-text version 0.1.1 is not as good as the output from pdftotext. e.g. pdftotext seems to handle line breaks and indents better. Here is also room for improvement.
