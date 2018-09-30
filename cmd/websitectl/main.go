package main

import (
	"errors"
	"log"
	"os"

	"github.com/Michael-F-Bryan/website"
	"github.com/urfave/cli"
)

func main() {
	var db *website.Database

	app := cli.NewApp()
	app.EnableBashCompletion = true

	app.Before = func(ctx *cli.Context) error {
		url := ctx.String("db")
		if url == "" {
			return errors.New("A database URL must be provided")
		}
		got, err := website.NewDatabase(url)
		if err != nil {
			return err
		}

		db = got
		return nil
	}

	app.Flags = []cli.Flag{
		cli.StringFlag{
			Name:   "db",
			Usage:  "The database URL to use when connecting to MongoDB",
			EnvVar: "DATABASE_URL",
		},
	}

	app.Commands = []cli.Command{
		{
			Name:    "create-user",
			Aliases: []string{"c"},
			Usage:   "Create a new user",
			Flags: []cli.Flag{
				cli.StringFlag{
					Name: "username, u",
				},
				cli.StringFlag{
					Name: "password, p",
				},
			},
			Action: func(c *cli.Context) error {
				username := c.String("username")
				password := c.String("password")
				if username == "" || password == "" {
					return errors.New("Both username and password must be specified")
				}

				_, err := db.CreateUser(username, password)
				return err
			},
		},

		{
			Name:    "delete-user",
			Aliases: []string{"d"},
			Usage:   "Delete a user",
			Flags: []cli.Flag{
				cli.StringFlag{
					Name: "username, u",
				},
			},
			Action: func(c *cli.Context) error {
				username := c.String("username")
				if username == "" {
					return errors.New("No username provided")
				}

				return db.DeleteUser(username)
			},
		},

		{
			Name:    "list-users",
			Aliases: []string{"l"},
			Usage:   "List all known users",
			Action: func(c *cli.Context) error {
				users, err := db.GetUsers()
				if err != nil {
					return err
				}

				log.Println("All Users:", users)
				return nil
			},
		},
	}

	if err := app.Run(os.Args); err != nil {
		log.Fatal(err)
	}
}
