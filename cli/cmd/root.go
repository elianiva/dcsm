package cmd

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:   "dcsm",
	Short: "Docker Compose Service Manager",
	Long: `Docker Compose Service Manager
			A simple way to manage your docker compose services similar to npm or any other package manager.`,
}

func Execute() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
