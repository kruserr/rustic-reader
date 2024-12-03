# curl --location --request POST 'http://localhost:3030/upload' \
# --header 'Content-Type: multipart/form-data' \
# --form 'file=@./test-data/pdf/pdfreference1.7old-1-50.pdf'

# curl -X POST localhost:3030/opened -H "Content-Type: application/json" -d '{"session_id":"d5cd462e-89ef-4267-9e35-5cc7a79b60eb","document_hash":"11431498542371153100"}'
# curl -X GET localhost:3030/opened/11431498542371153100

# curl -X POST localhost:3030/opened -H "Content-Type: application/json" -d '{"session_id":"d5cd462e-89ef-4267-9e35-5cc7a79b60eb","document_hash":"11431498542371153101"}'
curl -X GET localhost:3030/opened/11431498542371153101

# curl -X POST localhost:3030/progress -H "Content-Type: application/json" -d '{"session_id":"d5cd462e-89ef-4267-9e35-5cc7a79b60eb","document_hash":11431498542371153100,"offset":408,"total_lines":1274,"percentage":32.025117739403456}'
