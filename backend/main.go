package main

import "fmt"

func main() {
	fmt.Println("Hello World")

}

/*295  docker pull postgres:18.1-alpine3.23
  296  docker images
  297  docker images
  298  docker images
  299  docker images
  300  docker run --name postgres18 -p 5432:5432 -e POSTGRES_USER=root -e POSTGRES_PASSWORD=secret -d postgres:18.1-alpine3.23
  301  docker ps
  302  docker exec -it postgres18 psql -U root
  303  docker logs postgres18
  304  docker logs postgres18
  305  docker exec -it postgres18 psql -U root
  docker stop postgres18
  migrate create -ext sql -dir db/migration -seq init_schema
  (base) bjrivers@Brians-MacBook-Pro sabbaticalTraining % docker exec -
it postgres18 /bin/sh
(base) bjrivers@Brians-MacBook-Pro sabbaticalTraining % docker exec -
it postgres18 /bin/sh
/ # psql simple_bank
psql (18.1)
Type "help" for help.

simple_bank=#
ropdb simple_bank
migrate -path db/migration -database "postgresql://root:secret@localhost:5432/simple_bank?sslmode=disable" --verbose up
  306  docker logs postgres18*/
