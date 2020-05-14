package main
import (
    "fmt"
)

import "github.com/gin-gonic/gin"

func main() {
 httpPort := 9001
 r := gin.Default()
 r.GET("/ping", func(c *gin.Context) {
   c.JSON(200, gin.H{
     "value": "pong",
   })
 })
 r.Run(fmt.Sprintf(":%d", httpPort))
}
