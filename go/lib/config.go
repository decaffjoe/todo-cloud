package lib

import (
	"log"

	"github.com/spf13/viper"
)

type Config struct {
	// [key, value]
	GcpBucketName        [2]string
	GcpCredsFilePath     [2]string
	TodoFilePath         [2]string
	EncryptionPassphrase [2]string
}

func InitConfig() Config {
	// default values to empty strings
	// if key names are edited, must edit .update...() methods
	cfg := Config{
		GcpBucketName:        [2]string{"gcp_bucket_name", ""},
		GcpCredsFilePath:     [2]string{"gcp_creds_file_path", ""},
		TodoFilePath:         [2]string{"todo_file_path", ""},
		EncryptionPassphrase: [2]string{"encryption_passphrase", ""},
	}

	// read config file for (a) credentials filepath, (b) todo filepath, (c) encryption secret
	viper.SetConfigName("todo-cloud")
	viper.SetConfigType("toml")
	viper.AddConfigPath("$HOME/.config/todo-cloud")
	err := viper.ReadInConfig()
	if err != nil {
		if _, ok := err.(viper.ConfigFileNotFoundError); ok {
			// Config file not found; set all fields in config file to default empty string
			cfg.UpdateViperFromConfig()
			// TODO - figure out why this doesn't work
			viper.SafeWriteConfigAs("$HOME/.config/todo-cloud/todo-cloud.toml")
			log.Fatal("Update config file values at $HOME/.config/todo-cloud/todo-cloud.toml and run again")
		} else {
			// Config file was found but another error was produced
			log.Fatal(err)
		}
	}

	// update config object with read values
	cfg.UpdateConfigFromViper()

	return cfg
}

func (c *Config) UpdateConfigFromViper() {
	readConfigItem(&c.GcpBucketName)
	readConfigItem(&c.GcpCredsFilePath)
	readConfigItem(&c.TodoFilePath)
	readConfigItem(&c.EncryptionPassphrase)
}

func readConfigItem(cfgItem *[2]string) {
	cfgItem[1] = viper.Get(cfgItem[0]).(string)
}

func (c *Config) UpdateViperFromConfig() {
	setConfigItem(&c.GcpBucketName)
	setConfigItem(&c.GcpCredsFilePath)
	setConfigItem(&c.TodoFilePath)
	setConfigItem(&c.EncryptionPassphrase)
}

func setConfigItem(cfgItem *[2]string) {
	viper.Set(cfgItem[0], cfgItem[1])
}
