package dcsm_core

import (
	"fmt"
	"os"
	"testing"
)

func TestInitialiseNotExists(t *testing.T) {
	err := Initialise()
	if err != nil {
		t.Errorf("Initialise() failed with %s", err)
	}
	defer os.Remove("docker-compose.yml")

	// checks if the files exists or not
	if _, err := os.Stat("docker-compose.yml"); os.IsNotExist(err) {
		t.Errorf("Initialise() failed, expected 'docker-compose.yml' to exist")
	}

	// checks if the files has a correct version
	file, err := os.OpenFile("docker-compose.yml", os.O_RDONLY, 0666)
	if err != nil {
		t.Errorf("Initialise() failed with %s", err)
	}
	defer file.Close()

	var version string
	fmt.Fscanf(file, "version: %s\n\n", &version)
	if version != "3.7" {
		t.Errorf("Initialise() failed, expected '3.7', got '%s'", version)
	}
}

func TestInitialiseAlreadyExists(t *testing.T) {
	// create a file
	file, err := os.Create("docker-compose.yml")
	if err != nil {
		t.Errorf("Initialise() failed with %s", err)
	}
	defer file.Close()
	defer os.Remove("docker-compose.yml")

	err = Initialise()
	if err != nil {
		t.Errorf("Initialise() failed with %s", err)
	}
}
