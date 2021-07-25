Import-Module "$PSScriptRoot/../siquery"
$Tables = Get-siqList

Describe '<_> Schema Validation' -ForEach $Tables {
	Context '<Table> json schema validation' -ForEach @(@{ Table = $_ }) {
		BeforeAll {
			$SchemaPath = (Resolve-Path "$PSScriptRoot/../siquery/schema").Path
			$TableSchemaPath = Join-Path $SchemaPath "$Table.json"
		}

		It '<Table> has a json schema' { $TableSchemaPath | Should -Exist }

		It 'has a valid json schema' {
			$schema = Get-Content $TableSchemaPath -Raw
			$schema | Test-Json | Should -Be $true
		}

		It 'validates schema' {
			$TableData = Get-Siq $Table | Select-Object -First 1

			$schema = Get-Content $TableSchemaPath -Raw

			$json = $TableData | ConvertTo-Json -ErrorAction Stop
			$json | Test-Json -Schema $schema | Should -Be $true
		}
	}
}
