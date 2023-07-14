package dcsm_core

import (
	"os"
	"path"

	"github.com/pkg/errors"
)

// Initialise a new docker-compose configuration if it's not already initialised
func Initialise() error {
	hasConfig, err := hasConfig()
	if err != nil {
		return errors.Wrap(err, "failed to check for existing configuration")
	}

	if hasConfig {
		return nil
	}

	file, err := os.Create("docker-compose.yml")
	if err != nil {
		return errors.Wrap(err, "failed to create docker-compose.yml file")
	}
	defer file.Close()

	_, err = file.WriteString("version: 3.7\n\n")
	if err != nil {
		return errors.Wrap(err, "failed to write to docker-compose.yml file")
	}

	return nil
}

func hasConfig() (bool, error) {
	cwd, err := os.Getwd()
	if err != nil {
		return false, errors.Wrap(err, "failed to get current working directory")
	}

	// checks if file exists or not
	if _, err := os.Stat(path.Join(cwd, "docker-compose.yml")); os.IsNotExist(err) {
		return false, nil
	}

	return true, nil
}
