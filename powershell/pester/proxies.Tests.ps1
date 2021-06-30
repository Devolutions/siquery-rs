Import-Module "$PSScriptRoot/../siquery"

Describe 'proxies table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll { $allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json" }
			It 'has a schema' {
				$proxies = $allSchemas | Where-Object { $_.BaseName -eq 'proxies' }
				$proxies | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$proxies = $allSchemas | Where-Object { $_.BaseName -eq 'proxies' }
				$schema = Get-Content $proxies.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			It 'validates schema' {
				$proxies = Get-Siq 'proxies' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'proxies' }).FullName -Raw

				$json = $proxies | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
