package cmd

import (
	"dcsm_core"
	"fmt"
	"log"

	"github.com/spf13/cobra"
)

func init() {
	rootCmd.AddCommand(createCmd)
}

var createCmd = &cobra.Command{
	Use:   "create",
	Short: "Create a new service",
	Long: `Create a new service
			You can create a new service by using the create command.
			Example: dcsm create <service-name>
					 dcsm create postgre`,
	Run: func(cmd *cobra.Command, args []string) {
		err := dcsm_core.Initialise()
		if err != nil {
			log.Fatal(err)
		}
		fmt.Println("project has been initialised")
	},
}
