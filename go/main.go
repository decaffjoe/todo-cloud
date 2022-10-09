package main

import (
	"context"
	"fmt"
	"log"
	"os"

	"cloud.google.com/go/storage"
	"google.golang.org/api/option"

	"go-todo-cloud/lib"
)

func main() {
	// read config file
	cfg := lib.InitConfig()

	// read and encrypt todo file
	todoFile, err := os.ReadFile(cfg.TodoFilePath[1])
	if err != nil {
		log.Fatal(err)
	}
	encrypted := lib.Encrypt(todoFile, []byte(cfg.EncryptionPassphrase[1]))

	// authenticate to gcp
	ctx := context.Background()
	client, err := storage.NewClient(ctx, option.WithCredentialsFile(cfg.GcpCredsFilePath[1]))
	if err != nil {
		log.Fatal(err)
	}

	// write to bucket
	w := client.Bucket(cfg.GcpBucketName[1]).Object("todo").NewWriter(ctx)
	_, err = fmt.Fprintf(w, string(encrypted))
	if err != nil {
		log.Fatal(err)
	}

	// close connection
	err = w.Close()
	if err != nil {
		log.Fatal(err)
	}
}
