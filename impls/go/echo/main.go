package main

import (
	"fmt"
	"net/http"
	"strconv"
	
	"github.com/labstack/echo/v4"
)

func helloHandler(c echo.Context) error {
	idStr := c.Param("id")
	name := c.Param("name")
	
	id, err := strconv.Atoi(idStr)
	if err != nil {
		panic(err)
	}

	return c.String(http.StatusOK, fmt.Sprintf("Hello %s! id:%d", name, id))
}

func main() {
	e := echo.New()

	e.GET("/demo", func(c echo.Context) error {
		return c.String(http.StatusOK, "Hello")
	})

	e.GET("/demo/:id/:name/other.html", helloHandler)
	e.POST("/demo/:id/:name/other.html", helloHandler)
	e.GET("/demo/:id/:name/test.html", helloHandler)
	e.GET("/demo/:id/:name/index.html", helloHandler)

	e.Logger.Fatal(e.Start("127.0.0.1:8080"))
}
