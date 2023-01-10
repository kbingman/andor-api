GET http://localhost:3000/

HTTP/* 404

GET http://localhost:3000/people

HTTP/* 200
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$.results[0].name" == "Cassian Andor"
jsonpath "$.results[0].episode_ids[0]" == 2
jsonpath "$.results[1].name" == "Maarva Andor"
