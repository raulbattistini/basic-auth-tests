package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"log"
	"net/http"
	"os"
)

const port = 6969

func validateBasicAuthCredentials(w http.ResponseWriter, r *http.Request) error {
	expectedUsername := os.Getenv("USERNAME")
	expectedPassword := os.Getenv("PASSWORD")

	username, password, ok := r.BasicAuth()
	if !ok || username != expectedUsername || password != expectedPassword {
		log.Printf("invalid credentials; either username %s different from %s or password %s different from %s", username, expectedUsername, password, expectedPassword)
		return errors.New("username ou senha inválidos")
	}
	return nil
}

func post(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", 405)
		return
	} else {
		if err := validateBasicAuthCredentials(w, r); err != nil {
			log.Printf("invalid credentials")
			http.Error(w, "invalid credentials", 401)
			return
		}
		data := map[string]interface{}{
			"code":    200,
			"status":  "success",
			"message": "joia",
		}
		dataBytes, err := json.Marshal(data)
		if err != nil {
			// should be treating it
			http.Error(w, "internal server error", 500)
			return
		}
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(200)
		w.Write(dataBytes)
		return
	}
}

func main() {

	http.HandleFunc("/webhook-sample", post)
	log.Printf("server listening on port %d", port)
	addr := fmt.Sprintf("127.0.0.1:%d", port)
	if err := http.ListenAndServe(addr, nil); err != nil {
		log.Fatalf("fatal err %v serving on port %d", err, port)
	}
}
