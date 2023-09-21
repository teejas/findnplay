package main

import (
	"fmt"
	"log"
	"net/http"
	"os"

	"github.com/gin-gonic/gin"
	"github.com/urfave/cli/v2"
)

func startServer(c *cli.Context) error {
	fmt.Printf("dir: %s\n", c.String("dir"))
	fmt.Println("Starting HTTP server...")
	r := gin.Default()
	r.GET("/ping", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{
			"message": "pong",
		})
	})
	r.Run()
	return nil
}

func main() {
	app := cli.NewApp()
	app.Name = "findnplay"
	app.Usage = "provide a path to a directory containing audio files"
	app.Flags = []cli.Flag{
		&cli.StringFlag{
			Name:     "dir",
			Aliases:  []string{"d"},
			Usage:    "a path to a directory containing audio files",
			Required: true,
		},
	}
	app.Action = startServer

	err := app.Run(os.Args)
	if err != nil {
		log.Fatal(err)
	}
}
