package main

import (
	"net/http"
	"strconv"

	"github.com/gin-gonic/gin"
)

func helloHandler(c *gin.Context) {
	idStr := c.Param("id")
	name := c.Param("name")
	
	id, err := strconv.Atoi(idStr)
	if err != nil {
		panic(err)
	}

	c.String(http.StatusOK, "Hello %s! id:%d", name, id)
}

func main() {
	router := gin.New()
	router.Use(gin.Recovery())

	router.GET("/demo", func(c *gin.Context) {
		c.String(http.StatusOK, "Hello")
	})

	router.GET("/demo/:id/:name/other.html", helloHandler)
	router.POST("/demo/:id/:name/other.html", helloHandler)
	router.GET("/demo/:id/:name/test.html", helloHandler)
	router.GET("/demo/:id/:name/index.html", helloHandler)

	router.Run("127.0.0.1:8080")
}
