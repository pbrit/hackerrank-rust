package main

import (
	"flag"
	"io/ioutil"
	"net/http"
	"net/url"
	"os"
	"path"
	"strings"

	"fmt"

	"encoding/json"

	"github.com/BurntSushi/toml"
	"github.com/PuerkitoBio/goquery"
	_ "github.com/PuerkitoBio/goquery"
	"github.com/golang/glog"
	"github.com/paulche/utils"
)

const (
	inputSel  = "body div.challenge_sample_input div.msB.challenge_sample_input_body div pre"
	outputSel = "body div.challenge_sample_output div.msB.challenge_sample_output_body div pre"
)

// Config ...
type Config struct {
	ChallengePrefix string
}

var opOpt = flag.String("op", "submit", "Operation: submit or create")
var urlOpt = flag.String("url", "", "URL where a challenge is living")
var configOpt = flag.String("config", "config.toml", "Config file")

var config Config

func main() {
	flag.Parse()
	glog.V(3).Infof("Starting ...")

	fullConfigPath := path.Join(utils.MustString(os.Getwd()), *configOpt)
	glog.V(3).Infof("Reading config: %#v", fullConfigPath)

	configFileData := string(utils.MayAny(ioutil.ReadFile(fullConfigPath)).Otherwise(func(err error) {
		glog.Fatalf("Failed to read the content of file %#v", fullConfigPath)
	}).([]byte))

	utils.MaySkip(toml.Decode(configFileData, &config)).Otherwise(func(err error) {
		glog.Fatalf("Failed to parse the config file %#v", fullConfigPath)
	})

	switch *opOpt {
	case "submit":
		submit()
	case "create":
		create()
	default:
		glog.Fatalf("Unsupported operation: %#v, should be either submit or create.", *opOpt)
	}
}

func submit() {

}

func create() {
	parsedURL := utils.MayAny(url.Parse(*urlOpt)).Otherwise(func(err error) {
		glog.Fatalf("Failed to parse URL: '%v'. Original error: %v", *urlOpt, err)
	}).(*url.URL)

	pathArray := strings.Split(parsedURL.Path, "/")
	chID := pathArray[len(pathArray)-1]
	targetURL := fmt.Sprintf("%v/%v", config.ChallengePrefix, chID)

	glog.V(3).Infof("Reading the content of %v", targetURL)

	resp := utils.MayAny(http.Get(targetURL)).Otherwise(func(err error) {
		glog.Fatalf("Failed to make GET request to %v. Err: %#v", *urlOpt, err)
	}).(*http.Response)
	defer resp.Body.Close()

	jsonDocument := struct {
		Model struct {
			Body string `json:"body_html"`
		} `json:"model"`
	}{}

	jsonDecoder := json.NewDecoder(resp.Body)

	utils.May(jsonDecoder.Decode(&jsonDocument), func(err error) {
		glog.Fatalf("Failed to decode JSON document. Err: %v", err)
	})

	reader := strings.NewReader(jsonDocument.Model.Body)

	document := utils.MayAny(goquery.NewDocumentFromReader(reader)).Otherwise(func(err error) {
		glog.Fatalf("Failed to parse html document. Err: %v", err)
	}).(*goquery.Document)

	chInput := document.Find(inputSel).Text()
	chOutput := document.Find(outputSel).Text()

	glog.Infof("Input: %#v", chInput)
	glog.Infof("Output: %#v", chOutput)
}
