Import-Module "$PSScriptRoot/../siquery"

Describe 'etc_protocols table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll { $allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json" }
			It 'has a schema' {
				$etc_protocols = $allSchemas | Where-Object { $_.BaseName -eq 'etc_protocols' }
				$etc_protocols | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$etc_protocols = $allSchemas | Where-Object { $_.BaseName -eq 'etc_protocols' }
				$schema = Get-Content $etc_protocols.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			It 'validates schema' {
				$etc_protocols = Get-Siq 'etc_protocols' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'etc_protocols' }).FullName -Raw

				$json = $etc_protocols | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
