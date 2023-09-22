package main

import (
	"fmt"
	"log"
	"net/http"
	"os"
	"path/filepath"

	"github.com/gin-gonic/gin"
	"github.com/urfave/cli/v2"
)

func fetchSongs(dir string) ([]string, error) {
	// entries, err := os.ReadDir(dir)
	fmt.Println(dir)
	entries := []string{}
	err := filepath.Walk(dir,
		func(path string, info os.FileInfo, err error) error {
			if err != nil {
				return err
			}
			fmt.Println(path, info.Size())
			if path != dir {
				entries = append(entries, path[len(dir)+1:])
			}
			return nil
		})
	if err != nil {
		log.Fatal(err)
		return []string{}, err
	}
	return entries, nil
}

func startServer(c *cli.Context) error {
	fmt.Printf("dir: %s\n", c.String("dir"))
	songs, err := fetchSongs(c.String("dir"))
	if err != nil {
		log.Fatal("Failed to fetch songs")
		return err
	} else if len(songs) == 0 {
		fmt.Println("No songs found in specified directory")
		return nil
	}
	fmt.Println("Starting HTTP server...")
	r := gin.Default()
	r.LoadHTMLGlob("web/*.html")
	r.GET("/ping", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{
			"message": "pong",
		})
	})
	r.GET("/", func(c *gin.Context) {
		c.HTML(http.StatusOK, "index.html", nil)
	})
	r.GET("/songs", func(c *gin.Context) {
		// song_names := []string{}
		// for _, s := range songs {
		// 	song_names = append(song_names, s.Name())
		// }
		c.JSON(http.StatusOK, gin.H{
			"message": songs,
		})
	})
	r.StaticFS("/song", http.Dir(c.String("dir")))
	r.Run(":8080")
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
