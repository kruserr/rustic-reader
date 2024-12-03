# curl --location --request POST 'http://localhost:3030/upload' \
# --header 'Content-Type: multipart/form-data' \
# --form 'file=@./test-data/pdf/pdfreference1.7old-1-50.pdf'

curl -X POST localhost:3030/progress -H "Content-Type: application/json" -d '{"document_hash":11431498542371153100,"offset":408,"total_lines":1274,"percentage":32.025117739403456}'
